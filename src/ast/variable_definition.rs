use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct VariableDefinition {
	location: Location,
	name: String,
	value: Box<dyn ast::Node>,
}

impl VariableDefinition {
	pub fn new(location: Location, name: String, value: Box<dyn ast::Node>) -> Self {
		Self {
			location,
			name,
			value,
		}
	}
}

impl ast::Node for VariableDefinition {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		self.value.define_functions(ctx);
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let value = self.value.generate(ctx)?;
		let index = ctx.define_variable(self.name.clone(), hlir::ValueType::I64);

		Ok(hlir::Node::Assignment {
			variable: Box::new(hlir::Node::Local(index)),
			value: Box::new(value),
		})
	}
}
