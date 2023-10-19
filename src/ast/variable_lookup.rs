use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct VariableLookup {
	location: Location,
	identifier: String,
}

impl VariableLookup {
	pub fn new(location: Location, identifier: String) -> Self {
		Self {
			location,
			identifier,
		}
	}
}

impl ast::Node for VariableLookup {
	fn define_functions(&self, ctx: &mut hlir::Context) {}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		if let Some((index, _)) = ctx.find_variable(&self.identifier) {
			Ok(hlir::Node::Local(index))
		} else if let Some(index) = ctx.find_function(&self.identifier) {
			Ok(hlir::Node::Function(index))
		} else {
			Err(format!("cannot find variable '{}'", self.identifier))
		}
	}
}
