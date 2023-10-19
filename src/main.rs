mod ast;
mod hlir;
mod lexer;
mod llir;
mod location;
mod lowerer;
mod operator;
mod parser;
mod token;
mod x86_64;

pub use lexer::Lexer;
pub use location::Location;
pub use lowerer::Lowerer;
pub use operator::Operator;
pub use parser::Parser;
pub use token::Token;

use crate::ast::Node;

pub type Result<T> = core::result::Result<T, String>;

fn main() -> Result<()> {
	let lexer = Lexer::new("./test_script.y")?;
	let tokens = lexer.lex()?;

	let parser = Parser::new(tokens);
	let ast = parser.parse_global_scope()?;

	let mut ir_context = hlir::Context::new();
	ast.define_functions(&mut ir_context);

	let hlir = ast.generate(&mut ir_context)?;

	let mut lowerer = Lowerer::new();
	lowerer.lower(hlir);

	let llir = lowerer.get();

	let compiler = x86_64::Compiler::new();
	let executable = compiler.compile(llir);

	let ret = executable.call();
	println!("ret: {ret}");

	Ok(())
}
