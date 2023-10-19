use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct FunctionDefinition {
	location: Location,
	name: String,
	parameters: Vec<ast::ParameterDefinition>,
	return_type: usize,
	body: ast::Scope,
}

impl FunctionDefinition {
	pub fn new(
		location: Location,
		name: String,
		parameters: Vec<ast::ParameterDefinition>,
		return_type: usize,
		body: ast::Scope,
	) -> Self {
		Self {
			location,
			name,
			parameters,
			return_type,
			body,
		}
	}
}

impl ast::Node for FunctionDefinition {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		ctx.define_function(self.name.clone());
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		ctx.push_scope();

		let parameters = self
			.parameters
			.iter()
			.map(|x| x.generate(ctx))
			.collect::<Result<Vec<_>>>()?;

		let body = self.body.generate(ctx)?;
		let locals = ctx.local_variables().clone();

		ctx.pop_scope();

		Ok(hlir::Node::FunctionDefinition {
			body: Box::new(body),
			parameters,
			result: None,
			locals,
			is_entry_point: self.name == "main",
		})
	}
}
