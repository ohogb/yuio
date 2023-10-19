use crate::{hlir, llir};

pub struct Lowerer {
	functions: Vec<llir::Function>,
	locals: std::collections::HashMap<usize, llir::Register>,
}

impl Lowerer {
	pub fn new() -> Self {
		Self {
			functions: Vec::new(),
			locals: std::collections::HashMap::new(),
		}
	}

	pub fn lower(&mut self, node: hlir::Node) -> Option<llir::Register> {
		match node {
			hlir::Node::GlobalScope { functions } => {
				for i in functions {
					self.lower(i);
				}

				None
			}
			hlir::Node::FunctionDefinition {
				body,
				parameters,
				result: _,
				locals: _,
				is_entry_point,
			} => {
				let parameter_count = parameters.len();

				let function = llir::Function {
					is_entry_point,
					parameters: parameters
						.into_iter()
						.enumerate()
						.map(|(index, node)| {
							let hlir::Node::ParameterDefinition(typ) = node else {
								unreachable!();
							};

							self.locals.insert(index, llir::Register(index));

							match typ {
								hlir::ValueType::Unit => 0,
								hlir::ValueType::I64 => 8,
								hlir::ValueType::Boolean => 1,
							}
						})
						.collect::<Vec<_>>(),
					body: Vec::new(),
					register_count: parameter_count,
				};

				self.functions.push(function);

				self.lower(*body);
				self.locals.clear();

				None
			}
			hlir::Node::Block(x) => {
				for i in x {
					self.lower(i);
				}

				None
			}
			hlir::Node::If {
				condition,
				true_branch,
				false_branch: _,
			} => {
				let condition = self.lower(*condition).unwrap();

				let jmp = self.emit(llir::Node::JumpOnZero {
					condition,
					target: 0,
				});

				self.lower(*true_branch);

				let label = self.label();

				if let Some(llir::Node::JumpOnZero { target, .. }) = self.get_mut(jmp) {
					*target = label;
				} else {
					unreachable!();
				};

				None
			}
			hlir::Node::Call {
				function,
				arguments,
			} => {
				let dst = self.register();
				let node = llir::Node::Call {
					dst,
					function: match *function {
						hlir::Node::Function(x) => x,
						_ => unreachable!(),
					},
					arguments: arguments
						.into_iter()
						.map(|x| self.lower(x).unwrap())
						.collect::<Vec<_>>(),
				};

				self.emit(node);
				Some(dst)
			}
			hlir::Node::Ret { value } => {
				let node = llir::Node::Return {
					value: value.map(|x| self.lower(*x).unwrap()),
				};

				self.emit(node);
				None
			}
			hlir::Node::Assignment { variable, value } => {
				let node = llir::Node::Move {
					dst: self.lower(*variable).unwrap(),
					src: self.lower(*value).unwrap(),
				};

				self.emit(node);
				None
			}
			hlir::Node::Add { lhs, rhs } => {
				let dst = self.register();
				let node = llir::Node::Add {
					dst,
					lhs: self.lower(*lhs).unwrap(),
					rhs: self.lower(*rhs).unwrap(),
				};

				self.emit(node);
				Some(dst)
			}
			hlir::Node::Mul { lhs, rhs } => {
				let dst = self.register();
				let node = llir::Node::Mul {
					dst,
					lhs: self.lower(*lhs).unwrap(),
					rhs: self.lower(*rhs).unwrap(),
				};

				self.emit(node);
				Some(dst)
			}
			hlir::Node::Equals { lhs, rhs } => {
				let dst = self.register();
				let node = llir::Node::Equals {
					dst,
					lhs: self.lower(*lhs).unwrap(),
					rhs: self.lower(*rhs).unwrap(),
				};

				self.emit(node);
				Some(dst)
			}
			hlir::Node::I64(x) => {
				let dst = self.register();
				let node = llir::Node::MoveImmI64 { dst, imm: x };

				self.emit(node);
				Some(dst)
			}
			hlir::Node::Function(_) => {
				unreachable!();
			}
			hlir::Node::Local(x) => {
				if let Some(x) = self.locals.get(&x) {
					Some(*x)
				} else {
					let ret = self.register();
					self.locals.insert(x, ret);

					Some(ret)
				}
			}
			hlir::Node::ParameterDefinition(_) => {
				unreachable!();
			}
		}
	}

	pub fn get(self) -> Vec<llir::Function> {
		self.functions
	}

	fn register(&mut self) -> llir::Register {
		let func = self.functions.last_mut().unwrap();

		let ret = llir::Register(func.register_count);
		func.register_count += 1;

		ret
	}

	fn emit(&mut self, node: llir::Node) -> usize {
		let func = self.functions.last_mut().unwrap();
		let ret = func.body.len();

		func.body.push(node);
		ret
	}

	fn label(&self) -> usize {
		let func = self.functions.last().unwrap();
		func.body.len()
	}

	fn get_mut(&mut self, label: usize) -> Option<&mut llir::Node> {
		let func = self.functions.last_mut().unwrap();
		func.body.get_mut(label)
	}
}
