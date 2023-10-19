use crate::{Location, Token};

pub struct Lexer {
	file_name: String,
	line: usize,
	column: usize,
	file_contents: String,
	current_index: usize,
}

impl Lexer {
	pub fn new(file_name: impl AsRef<str>) -> Result<Self, String> {
		let file_contents =
			std::fs::read_to_string(file_name.as_ref()).map_err(|x| x.to_string())?;

		Ok(Self {
			file_name: String::from(file_name.as_ref()),
			line: 1,
			column: 1,
			file_contents,
			current_index: 0,
		})
	}

	pub fn lex(mut self) -> Result<Vec<(Token, Location)>, String> {
		let mut ret = vec![];

		loop {
			self.skip_whitespace();

			if self.current_index == self.file_contents.len() {
				break;
			}

			let token = match self.get_word() {
				Some(token @ "if") => {
					let location = self.advance(token.len());
					Some((Token::If, location))
				}
				Some(token @ "fn") => {
					let location = self.advance(token.len());
					Some((Token::Fn, location))
				}
				Some(token @ "return") => {
					let location = self.advance(token.len());
					Some((Token::Return, location))
				}
				Some(token @ "let") => {
					let location = self.advance(token.len());
					Some((Token::Let, location))
				}
				Some(token) if !token.is_empty() => {
					let token = String::from(token);
					let location = self.advance(token.len());

					if token.parse::<i64>().is_ok() {
						Some((Token::Number(token), location))
					} else {
						Some((Token::Identifier(token), location))
					}
				}
				_ => None,
			};

			if let Some(token) = token {
				ret.push(token);
				continue;
			}

			let token = match self.get_char(0) {
				Some('(') => {
					let location = self.advance(1);
					Some((Token::OpeningParen, location))
				}
				Some(')') => {
					let location = self.advance(1);
					Some((Token::ClosingParen, location))
				}
				Some('{') => {
					let location = self.advance(1);
					Some((Token::OpeningCurly, location))
				}
				Some('}') => {
					let location = self.advance(1);
					Some((Token::ClosingCurly, location))
				}
				Some(':') => {
					let location = self.advance(1);
					Some((Token::Colon, location))
				}
				Some(';') => {
					let location = self.advance(1);
					Some((Token::SemiColon, location))
				}
				Some(',') => {
					let location = self.advance(1);
					Some((Token::Comma, location))
				}
				Some('+') => {
					let location = self.advance(1);
					Some((Token::Plus, location))
				}
				Some('-') => {
					let location = self.advance(1);
					Some((Token::Minus, location))
				}
				Some('*') => {
					let location = self.advance(1);
					Some((Token::Star, location))
				}
				Some('/') => {
					let location = self.advance(1);
					Some((Token::Slash, location))
				}
				Some('=') => {
					let location = self.advance(1);
					Some((Token::Equals, location))
				}
				Some('!') => {
					let location = self.advance(1);
					Some((Token::ExclamationMark, location))
				}
				Some('?') => {
					let location = self.advance(1);
					Some((Token::QuestionMark, location))
				}
				_ => None,
			};

			if let Some(token) = token {
				ret.push(token);
				continue;
			}

			return Err(format!(
				"cannot lex {:?}",
				&self.file_contents[self.current_index..]
			))?;
		}

		Ok(ret)
	}

	fn get_while(&self, callback: impl Fn(Option<char>) -> Option<bool>) -> Option<&'_ str> {
		let mut index = 0;

		while callback(self.file_contents.chars().nth(self.current_index + index))? {
			index += 1;
		}

		Some(&self.file_contents[self.current_index..self.current_index + index])
	}

	fn get_word(&self) -> Option<&'_ str> {
		self.get_while(|x| Some(x?.is_ascii_alphanumeric() || x? == '_'))
	}

	fn get_char(&self, offset: usize) -> Option<char> {
		self.file_contents.chars().nth(self.current_index + offset)
	}

	fn advance_while(&mut self, mut callback: impl FnMut(char) -> bool) -> Location {
		let ret = Location {
			file_name: self.file_name.clone(),
			line: self.line,
			column: self.column,
		};

		loop {
			let c = self.file_contents.chars().nth(self.current_index);

			let Some(c) = c else {
				break;
			};

			if !callback(c) {
				break;
			}

			if c == '\n' {
				self.line += 1;
				self.column = 1;
			} else {
				self.column += 1;
			}

			self.current_index += 1;
		}

		ret
	}

	fn advance(&mut self, amount: usize) -> Location {
		let mut index = 0;

		self.advance_while(|_| {
			index += 1;
			index <= amount
		})
	}

	fn skip_whitespace(&mut self) {
		self.advance_while(|c| c.is_ascii_whitespace());
	}
}
