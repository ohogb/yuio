use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct Call {
	location: Location,
	function: Box<dyn ast::Node>,
	arguments: Vec<Box<dyn ast::Node>>,
}

impl Call {
	pub fn new(
		location: Location,
		function: Box<dyn ast::Node>,
		arguments: Vec<Box<dyn ast::Node>>,
	) -> Self {
		Self {
			location,
			function,
			arguments,
		}
	}
}

impl ast::Node for Call {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		self.function.define_functions(ctx);

		for i in &self.arguments {
			i.define_functions(ctx);
		}
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let function = self.function.generate(ctx)?;

		let arguments = self
			.arguments
			.iter()
			.map(|x| x.generate(ctx))
			.collect::<Result<Vec<_>>>()?;

		Ok(hlir::Node::Call {
			function: Box::new(function),
			arguments,
		})
	}
}
