use crate::{ast, hlir, Result};

#[derive(Debug)]
pub struct GlobalScope {
	functions: Vec<ast::FunctionDefinition>,
}

impl GlobalScope {
	pub fn new() -> Self {
		Self {
			functions: Vec::new(),
		}
	}

	pub fn push_function(&mut self, function: ast::FunctionDefinition) {
		self.functions.push(function);
	}
}

impl ast::Node for GlobalScope {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		for i in &self.functions {
			i.define_functions(ctx);
		}
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let mut functions = Vec::new();

		for i in &self.functions {
			functions.push(i.generate(ctx)?);
		}

		Ok(hlir::Node::GlobalScope { functions })
	}
}
