use crate::{ast, hlir, Location, Result};

#[derive(Debug)]
pub struct If {
	location: Location,
	condition: Box<dyn ast::Node>,
	true_branch: ast::Scope,
	false_branch: Option<ast::Scope>,
}

impl If {
	pub fn new(
		location: Location,
		condition: Box<dyn ast::Node>,
		true_branch: ast::Scope,
		false_branch: Option<ast::Scope>,
	) -> Self {
		Self {
			location,
			condition,
			true_branch,
			false_branch,
		}
	}
}

impl ast::Node for If {
	fn define_functions(&self, ctx: &mut hlir::Context) {
		self.condition.define_functions(ctx);
		self.true_branch.define_functions(ctx);

		if let Some(x) = &self.false_branch {
			x.define_functions(ctx);
		}
	}

	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node> {
		let condition = self.condition.generate(ctx)?;
		let true_branch = self.true_branch.generate(ctx)?;

		if condition.get_type() != hlir::ValueType::Boolean {
			Err(format!("expected Boolean, got {:?}", condition.get_type()))?
		}

		Ok(hlir::Node::If {
			condition: Box::new(condition),
			true_branch: Box::new(true_branch),
			false_branch: None,
		})
	}
}
