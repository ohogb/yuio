use crate::llir;

#[derive(Debug)]
pub struct Function {
	pub is_entry_point: bool,
	pub parameters: Vec<usize>,
	pub body: Vec<llir::Node>,
	pub register_count: usize,
}

impl Function {
	pub fn new(is_entry_point: bool, parameters: Vec<usize>) -> Self {
		Self {
			is_entry_point,
			parameters,
			body: Vec::new(),
			register_count: 0,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Register(pub usize);

#[derive(Debug)]
pub enum Node {
	Move {
		dst: llir::Register,
		src: llir::Register,
	},
	MoveImmI64 {
		dst: llir::Register,
		imm: i64,
	},
	Jump {
		target: usize,
	},
	JumpOnZero {
		condition: llir::Register,
		target: usize,
	},
	Add {
		dst: llir::Register,
		lhs: llir::Register,
		rhs: llir::Register,
	},
	Mul {
		dst: llir::Register,
		lhs: llir::Register,
		rhs: llir::Register,
	},
	Equals {
		dst: llir::Register,
		lhs: llir::Register,
		rhs: llir::Register,
	},
	Return {
		value: Option<llir::Register>,
	},
	Call {
		dst: llir::Register,
		function: usize,
		arguments: Vec<llir::Register>,
	},
}
