use crate::{ast, hlir, Result};

pub trait Node: std::fmt::Debug {
	fn define_functions(&self, ctx: &mut hlir::Context);
	fn generate(&self, ctx: &mut hlir::Context) -> Result<hlir::Node>;

	fn type_check(&self) -> Result<()> {
		Ok(())
	}
}
