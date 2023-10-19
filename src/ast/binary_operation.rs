use crate::{ast, hlir, Location, Operator, Result};

#[derive(Debug)]
pub struct BinaryOperation {
	location: Location,
	lhs: Box<dyn ast::Node>,
	rhs: Box<dyn ast::Node>,
	op: Operator,
}

impl BinaryOperation {
	pub fn new(
		location: Location,
		lhs: Box<dyn ast::Node>,
		rhs: Box<dyn ast::Node>,
		op: Operator,
	) -> Self {
		Self {
			location,
			lhs,
			rhs,
			op,
		}
	}
}

impl ast::Node for BinaryOperation {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		self.lhs.define_functions(ctx);
		self.rhs.define_functions(ctx);
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let lhs = Box::new(self.lhs.generate(ctx)?);
		let rhs = Box::new(self.rhs.generate(ctx)?);

		if lhs.get_type() != rhs.get_type() {
			Err(format!(
				"cannot do {:?} {:?} {:?}",
				lhs.get_type(),
				self.op,
				rhs.get_type()
			))?;
		}

		Ok(match self.op {
			Operator::Add => hlir::Node::Add { lhs, rhs },
			Operator::Sub => todo!(),
			Operator::Mul => hlir::Node::Mul { lhs, rhs },
			Operator::Div => todo!(),
			Operator::Assignment => hlir::Node::Assignment {
				variable: lhs,
				value: rhs,
			},
			Operator::Equal => hlir::Node::Equals { lhs, rhs },
			Operator::NotEqual => todo!(),
		})
	}
}
