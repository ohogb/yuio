use crate::{ast, Location, Operator, Result, Token};

pub struct Parser {
	tokens: std::collections::VecDeque<(Token, Location)>,
}

impl Parser {
	pub fn new(tokens: Vec<(Token, Location)>) -> Self {
		Self {
			tokens: tokens.into(),
		}
	}

	pub fn parse_global_scope(mut self) -> Result<ast::GlobalScope> {
		let mut global_scope = ast::GlobalScope::new();

		while !self.tokens.is_empty() {
			if let Some(function) = self.try_parse_function_definition()? {
				global_scope.push_function(function);
				continue;
			}

			Err(format!(
				"cannot parse {:?} at global scope {}",
				self.tokens,
				self.tokens.is_empty()
			))?;
		}

		Ok(global_scope)
	}

	fn parse_scope(mut self) -> Result<ast::Scope> {
		let mut nodes: Vec<Box<dyn ast::Node>> = Vec::new();

		while !self.tokens.is_empty() {
			if let Some(node) = self.try_parse_if()? {
				nodes.push(Box::new(node));
				continue;
			}

			if let Some(node) = self.try_parse_variable_definition()? {
				nodes.push(Box::new(node));
				continue;
			}

			if let Some(node) = self.try_parse_return()? {
				nodes.push(Box::new(node));
				continue;
			}

			let value = self.pop_while(|x| {
				let Some(token) = x else {
					return Err(format!("expected expression, got nothing"))?;
				};

				Ok(*token != Token::SemiColon)
			})?;

			let value = Self::new(value).parse_expression()?;

			match self.tokens.pop_front() {
				Some((Token::SemiColon, _)) => {}
				x => return Err(format!("expected SemiColon, got {x:?}")),
			}

			nodes.push(value);

			// Err(format!("cannot parse {:?} in scope", self.tokens))?;
		}

		Ok(ast::Scope::new(
			Location {
				file_name: String::from(""),
				line: 0,
				column: 0,
			},
			nodes,
		))
	}

	fn parse_expression(&mut self) -> Result<Box<dyn ast::Node>> {
		let mut value = self.parse_value()?;
		value = self.try_parse_function_call(value)?;

		while !self.tokens.is_empty() {
			let Some(op) = self.parse_operator(0)? else {
				return Err("asdfasdf").unwrap();
			};

			value = self.parse_operation(value, op)?;
		}

		Ok(value)
	}

	fn parse_value(&mut self) -> Result<Box<dyn ast::Node>> {
		match self.tokens.pop_front() {
			Some((Token::Number(num), location)) => Ok(Box::new(ast::Integer::new(
				location,
				num.parse::<i64>().map_err(|x| x.to_string())?,
			))),
			Some((Token::Identifier(ident), location)) => {
				Ok(Box::new(ast::VariableLookup::new(location, ident)))
			}
			x => Result::Err(format!("expr: {x:?}")).unwrap(),
		}
	}

	fn parse_operator(&mut self, precedence: u8) -> Result<Option<Operator>> {
		let (op, size) = match self.tokens.front() {
			Some((Token::Plus, _)) => (Operator::Add, 1),
			Some((Token::Minus, _)) => (Operator::Sub, 1),
			Some((Token::Star, _)) => (Operator::Mul, 1),
			Some((Token::Slash, _)) => (Operator::Div, 1),
			Some((Token::Equals, _)) => match self.tokens.front() {
				Some((Token::Equals, _)) => (Operator::Equal, 2),
				_ => (Operator::Assignment, 1),
			},
			Some((x @ Token::ExclamationMark, _)) => match self.tokens.front() {
				Some((Token::Equals, _)) => (Operator::NotEqual, 2),
				_ => Err(format!("failed to parse {x:?} into an operator"))?,
			},
			_ => return Ok(None),
		};

		Ok(if op.precedence() >= precedence {
			for _ in 0..size {
				self.tokens.pop_front();
			}

			Some(op)
		} else {
			None
		})
	}

	fn parse_operation(
		&mut self,
		lhs: Box<dyn ast::Node>,
		op: Operator,
	) -> Result<Box<dyn ast::Node>> {
		let mut rhs = self.parse_value()?;

		rhs = self.try_parse_function_call(rhs)?;

		if let Some(x) = self.parse_operator(op.precedence())? {
			rhs = self.parse_operation(rhs, x)?;
		}

		Ok(Box::new(ast::BinaryOperation::new(
			Location {
				file_name: "".into(),
				line: 0,
				column: 0,
			},
			lhs,
			rhs,
			op,
		)))
	}

	fn parse_parameter(&mut self) -> Result<ast::ParameterDefinition> {
		let (name, name_location) = match self.tokens.pop_front() {
			Some((Token::Identifier(name), location)) => (name, location),
			x => Err(format!("expected Identifier, got {x:?}"))?,
		};

		match self.tokens.pop_front() {
			Some((Token::Colon, _)) => {}
			x => Err(format!("expected Colon, got {x:?}"))?,
		}

		let (typ, _) = match self.tokens.pop_front() {
			Some((Token::Identifier(typ), location)) => (typ, location),
			x => Err(format!("expected Identifier, got {x:?}"))?,
		};

		Ok(ast::ParameterDefinition::new(name_location, name, typ))
	}

	fn try_parse_function_call(&mut self, node: Box<dyn ast::Node>) -> Result<Box<dyn ast::Node>> {
		let Some((Token::OpeningParen, location)) = self.tokens.front() else {
			return Ok(node);
		};

		let location = location.clone();

		let arguments = self.pop_scope(Token::OpeningParen, Token::ClosingParen)?;
		let argument = Self::new(arguments).parse_expression()?;

		Ok(Box::new(ast::Call::new(location, node, vec![argument])))
	}

	fn try_parse_function_definition(&mut self) -> Result<Option<ast::FunctionDefinition>> {
		let Some((Token::Fn, _)) = self.tokens.front() else {
			return Ok(None);
		};

		let (_, location) = self.tokens.pop_front().unwrap();

		let name = match self.tokens.pop_front() {
			Some((Token::Identifier(name), _)) => name,
			x => return Err(format!("expected Identifier, got {x:?}")),
		};

		let parameters = self.pop_scope(Token::OpeningParen, Token::ClosingParen)?;
		let mut parameter_parser = Self::new(parameters);
		let mut parameters = Vec::new();

		loop {
			let parameter = parameter_parser.pop_while(|x| match x {
				Some(x) => Ok(*x != Token::Comma),
				None => Ok(false),
			})?;

			if parameter.is_empty() {
				break;
			}

			match parameter_parser.tokens.pop_front() {
				Some((Token::Comma, _)) => {}
				Some(x) => Err(format!("expected Comma, got {x:?}"))?,
				_ => {}
			}

			parameters.push(Self::new(parameter).parse_parameter()?);

			if parameter_parser.tokens.is_empty() {
				break;
			}
		}

		let body = self.pop_scope(Token::OpeningCurly, Token::ClosingCurly)?;

		let body = Self::new(body).parse_scope()?;

		Ok(Some(ast::FunctionDefinition::new(
			location, name, parameters, 0, body,
		)))
	}

	fn try_parse_if(&mut self) -> Result<Option<ast::If>> {
		let Some((Token::If, _)) = self.tokens.front() else {
			return Ok(None);
		};

		let (_, location) = self.tokens.pop_front().unwrap();

		let expression_tokens = self.pop_while(|x| {
			let Some(token) = x else {
				return Err(format!("expected expression, got nothing"))?;
			};

			Ok(*token != Token::OpeningCurly)
		})?;

		let expression = Self::new(expression_tokens).parse_expression()?;

		let true_branch_tokens = self.pop_scope(Token::OpeningCurly, Token::ClosingCurly)?;
		let true_branch = Self::new(true_branch_tokens).parse_scope()?;

		Ok(Some(ast::If::new(location, expression, true_branch, None)))
	}

	fn try_parse_variable_definition(&mut self) -> Result<Option<ast::VariableDefinition>> {
		let Some((Token::Let, _)) = self.tokens.front() else {
			return Ok(None);
		};

		let (_, location) = self.tokens.pop_front().unwrap();

		let name = match self.tokens.pop_front() {
			Some((Token::Identifier(name), _)) => name,
			x => return Err(format!("expected Identifier, got {x:?}")),
		};

		match self.tokens.pop_front() {
			Some((Token::Equals, _)) => {}
			x => return Err(format!("expected Equals, got {x:?}")),
		}

		let value = self.pop_while(|x| {
			let Some(token) = x else {
				return Err(format!("expected expression, got nothing"))?;
			};

			Ok(*token != Token::SemiColon)
		})?;

		let value = Self::new(value).parse_expression()?;

		match self.tokens.pop_front() {
			Some((Token::SemiColon, _)) => {}
			x => return Err(format!("expected SemiColon, got {x:?}")),
		}

		Ok(Some(ast::VariableDefinition::new(location, name, value)))
	}

	fn try_parse_return(&mut self) -> Result<Option<ast::Return>> {
		let Some((Token::Return, _)) = self.tokens.front() else {
			return Ok(None);
		};

		let (_, location) = self.tokens.pop_front().unwrap();

		let value = self.pop_while(|x| {
			let Some(token) = x else {
				return Err(format!("expected expression, got nothing"))?;
			};

			Ok(*token != Token::SemiColon)
		})?;

		let value = if value.is_empty() {
			None
		} else {
			Some(Self::new(value).parse_expression()?)
		};

		match self.tokens.pop_front() {
			Some((Token::SemiColon, _)) => {}
			x => return Err(format!("expected SemiColon, got {x:?}")),
		}

		Ok(Some(ast::Return::new(location, value)))
	}

	fn pop_while(
		&mut self,
		mut predicate: impl FnMut(Option<&Token>) -> Result<bool>,
	) -> Result<Vec<(Token, Location)>> {
		let mut ret = vec![];

		while predicate(self.tokens.front().map(|x| &x.0))? {
			ret.push(self.tokens.pop_front().unwrap());
		}

		Ok(ret)
	}

	fn pop_scope(&mut self, open: Token, close: Token) -> Result<Vec<(Token, Location)>> {
		let first = self
			.tokens
			.pop_front()
			.ok_or_else(|| format!("expected {open:?}, got nothing"))?;

		if first.0 != open {
			Err(format!("expected {open:?}, got {:?}", first.0))?;
		}

		let mut diff = 1;

		let ret = self.pop_while(|x| {
			let Some(token) = x else {
				return Err(format!("expected {close:?}, got nothing"))?;
			};

			if *token == open {
				diff += 1;
			} else if *token == close {
				diff -= 1;
			}

			Ok(diff > 0)
		})?;

		let last = self
			.tokens
			.pop_front()
			.ok_or_else(|| format!("expected {close:?}, got nothing"))?;

		if last.0 != close {
			Err(format!("expected {close:?}, got {:?}", last.0))?
		}

		Ok(ret)
	}
}
