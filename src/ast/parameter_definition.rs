use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct ParameterDefinition {
	location: Location,
	name: String,
	typ: String,
}

impl ParameterDefinition {
	pub fn new(location: Location, name: String, typ: String) -> Self {
		Self {
			location,
			name,
			typ,
		}
	}
}

impl ast::Node for ParameterDefinition {
	fn define_functions(&self, ctx: &mut hlir::Context) {}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		ctx.define_variable(self.name.clone(), hlir::ValueType::I64);
		Ok(hlir::Node::ParameterDefinition(hlir::ValueType::I64))
	}
}
