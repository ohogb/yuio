use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct Return {
	location: Location,
	value: Option<Box<dyn ast::Node>>,
}

impl Return {
	pub fn new(location: Location, value: Option<Box<dyn ast::Node>>) -> Self {
		Self { location, value }
	}
}

impl ast::Node for Return {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		if let Some(x) = &self.value {
			x.define_functions(ctx);
		}
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let value = if let Some(x) = &self.value {
			Some(Box::new(x.generate(ctx)?))
		} else {
			None
		};

		Ok(hlir::Node::Ret { value })
	}
}
