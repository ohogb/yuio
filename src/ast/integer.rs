use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct Integer {
	location: Location,
	value: i64,
}

impl Integer {
	pub fn new(location: Location, value: i64) -> Self {
		Self { location, value }
	}
}

impl ast::Node for Integer {
	fn define_functions(&self, ctx: &mut hlir::Context) {}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		Ok(hlir::Node::I64(self.value))
	}
}
