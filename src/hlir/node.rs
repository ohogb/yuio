use crate::hlir;

#[derive(Debug)]
pub enum Node {
	GlobalScope {
		functions: Vec<Self>,
	},
	FunctionDefinition {
		body: Box<Self>,
		parameters: Vec<Self>,
		result: Option<hlir::ValueType>,
		locals: Vec<hlir::ValueType>,
		is_entry_point: bool,
	},
	Block(Vec<Self>),
	If {
		condition: Box<Self>,
		true_branch: Box<Self>,
		false_branch: Option<Box<Self>>,
	},
	Call {
		function: Box<Self>,
		arguments: Vec<Self>,
	},
	Ret {
		value: Option<Box<Self>>,
	},
	Assignment {
		variable: Box<Self>,
		value: Box<Self>,
	},
	Add {
		lhs: Box<Self>,
		rhs: Box<Self>,
	},
	Mul {
		lhs: Box<Self>,
		rhs: Box<Self>,
	},
	Equals {
		lhs: Box<Self>,
		rhs: Box<Self>,
	},
	I64(i64),
	Function(usize),
	Local(usize),
	ParameterDefinition(hlir::ValueType),
}

impl Node {
	pub fn get_type(&self) -> hlir::ValueType {
		match self {
			Node::GlobalScope { .. } => hlir::ValueType::Unit,
			Node::FunctionDefinition { .. } => hlir::ValueType::Unit,
			Node::Block(_) => hlir::ValueType::Unit,
			Node::If { .. } => hlir::ValueType::Unit,
			Node::Call { function, .. } => match **function {
				Node::Function(_) => hlir::ValueType::I64,
				_ => unreachable!(),
			},
			Node::Ret { .. } => hlir::ValueType::Unit,
			Node::Assignment { .. } => hlir::ValueType::Unit,
			Node::Add { lhs, rhs } => {
				let left = lhs.get_type();
				let right = rhs.get_type();

				assert!(left == right);
				left
			}
			Node::Mul { lhs, rhs } => {
				let left = lhs.get_type();
				let right = rhs.get_type();

				assert!(left == right);
				left
			}
			Node::Equals { lhs, rhs } => {
				let left = lhs.get_type();
				let right = rhs.get_type();

				assert!(left == right);
				hlir::ValueType::Boolean
			}
			Node::I64(_) => hlir::ValueType::I64,
			Node::Function(_) => hlir::ValueType::Unit,
			Node::Local(_) => hlir::ValueType::I64,
			Node::ParameterDefinition(_) => hlir::ValueType::I64,
		}
	}
}
