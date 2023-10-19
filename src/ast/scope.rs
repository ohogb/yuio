use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct Scope {
	location: Location,
	nodes: Vec<Box<dyn ast::Node>>,
}

impl Scope {
	pub fn new(location: Location, nodes: Vec<Box<dyn ast::Node>>) -> Self {
		Self { location, nodes }
	}
}

impl ast::Node for Scope {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		for i in &self.nodes {
			i.define_functions(ctx);
		}
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let mut ret = Vec::new();
		ctx.push_scope();

		for i in &self.nodes {
			ret.push(i.generate(ctx)?);
		}

		ctx.pop_scope();
		Ok(hlir::Node::Block(ret))
	}
}
