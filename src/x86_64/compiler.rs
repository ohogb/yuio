use crate::{llir, x86_64::Executable};

pub struct Compiler {
	output: Vec<u8>,
	positions: Vec<usize>,
	function_positions: Vec<usize>,
	branch_fixups: Vec<(usize, usize)>,
	function_fixups: Vec<(usize, usize)>,
	entry_point_offset: usize,
}

impl Compiler {
	pub fn new() -> Self {
		Self {
			output: Vec::new(),
			positions: Vec::new(),
			function_positions: Vec::new(),
			branch_fixups: Vec::new(),
			function_fixups: Vec::new(),
			entry_point_offset: 0,
		}
	}

	pub fn compile(mut self, functions: Vec<llir::Function>) -> Executable {
		for i in functions {
			if i.is_entry_point {
				self.entry_point_offset = self.output.len();
			}

			self.function_positions.push(self.output.len());

			self.emit([0x55]);

			self.emit([0x48, 0x89, 0xE5]);

			self.emit([0x48, 0x81, 0xEC]);
			self.emit(((i.register_count * 8) as u32).to_ne_bytes());

			for (index, _size) in i.parameters.into_iter().enumerate() {
				match index {
					0 => {
						// mov [rsp + (index * 8)], rdi
						self.emit([0x48, 0x89, 0xBC, 0x24]);
						self.emit(((index * 8) as u32).to_ne_bytes());
					}
					_ => todo!(),
				}
			}

			for i in i.body {
				self.positions.push(self.output.len());
				self.compile_node(i);
			}

			for (position, target) in self.branch_fixups.clone() {
				let target = self.positions.get(target).unwrap();
				self.encode_relative_32(position, *target);
			}

			self.branch_fixups.clear();
			self.positions.clear();
		}

		for (position, target) in self.function_fixups.clone() {
			let target = self.function_positions.get(target).unwrap();
			self.encode_relative_32(position, *target);
		}

		for i in &self.output {
			println!("{i:02X}");
		}

		Executable::new(self.output, self.entry_point_offset)
	}

	fn compile_node(&mut self, node: llir::Node) {
		match node {
			llir::Node::Move { dst, src } => {
				// mov rax, [rsp + src * 8]
				self.emit([0x48, 0x8B, 0x84, 0x24]);
				self.emit(((src.0 * 8) as u32).to_ne_bytes());

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
			llir::Node::MoveImmI64 { dst, imm } => {
				// mov rax, imm
				self.emit([0x48, 0xB8]);
				self.emit(imm.to_ne_bytes());

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
			llir::Node::Jump { target } => {
				// jmp target
				self.emit([0xE9]);

				let pos = self.output.len();
				self.emit([0x00, 0x00, 0x00, 0x00]);

				self.branch_fixups.push((pos, target));
			}
			llir::Node::JumpOnZero { condition, target } => {
				// mov rax, [rsp + condition * 8]
				self.emit([0x48, 0x8B, 0x84, 0x24]);
				self.emit(((condition.0 * 8) as u32).to_ne_bytes());

				// test rax, rax
				self.emit([0x48, 0x85, 0xC0]);

				// jz target
				self.emit([0x0F, 0x84]);

				let pos = self.output.len();
				self.emit([0x00, 0x00, 0x00, 0x00]);

				self.branch_fixups.push((pos, target));
			}
			llir::Node::Add { dst, lhs, rhs } => {
				// mov rax, [rsp + lhs * 8]
				self.emit([0x48, 0x8B, 0x84, 0x24]);
				self.emit(((lhs.0 * 8) as u32).to_ne_bytes());

				// mov rcx, [rsp + rhs * 8]
				self.emit([0x48, 0x8B, 0x8C, 0x24]);
				self.emit(((rhs.0 * 8) as u32).to_ne_bytes());

				// add rax, rcx
				self.emit([0x48, 0x01, 0xC8]);

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
			llir::Node::Mul { dst, lhs, rhs } => {
				// mov rax, [rsp + lhs * 8]
				self.emit([0x48, 0x8B, 0x84, 0x24]);
				self.emit(((lhs.0 * 8) as u32).to_ne_bytes());

				// mov rcx, [rsp + rhs * 8]
				self.emit([0x48, 0x8B, 0x8C, 0x24]);
				self.emit(((rhs.0 * 8) as u32).to_ne_bytes());

				// mul rcx
				self.emit([0x48, 0xF7, 0xE1]);

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
			llir::Node::Equals { dst, lhs, rhs } => {
				// mov rax, [rsp + lhs * 8]
				self.emit([0x48, 0x8B, 0x84, 0x24]);
				self.emit(((lhs.0 * 8) as u32).to_ne_bytes());

				// mov rcx, [rsp + rhs * 8]
				self.emit([0x48, 0x8B, 0x8C, 0x24]);
				self.emit(((rhs.0 * 8) as u32).to_ne_bytes());

				// cmp rax, rcx
				self.emit([0x48, 0x39, 0xC8]);

				// sete al
				self.emit([0x0F, 0x94, 0xC0]);

				// movzx rax, al
				self.emit([0x48, 0x0F, 0xB6, 0xC0]);

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
			llir::Node::Return { value } => {
				if let Some(value) = value {
					// mov rax, [rsp + lhs * 8]
					self.emit([0x48, 0x8B, 0x84, 0x24]);
					self.emit(((value.0 * 8) as u32).to_ne_bytes());
				}

				// mov rsp, rbp
				self.emit([0x48, 0x89, 0xEC]);

				// pop rbp
				self.emit([0x5D]);

				// ret
				self.emit([0xC3]);
			}
			llir::Node::Call {
				dst,
				function,
				arguments,
			} => {
				for (index, register) in arguments.iter().enumerate() {
					match index {
						0 => {
							// mov rdi, [rsp + lhs * 8]
							self.emit([0x48, 0x8B, 0xBC, 0x24]);
							self.emit(((register.0 * 8) as u32).to_ne_bytes());
						}
						_ => todo!(),
					}
				}

				// call function
				self.emit([0xE8]);

				let pos = self.output.len();
				self.emit([0x00, 0x00, 0x00, 0x00]);

				self.function_fixups.push((pos, function));

				// mov [rsp + dst * 8], rax
				self.emit([0x48, 0x89, 0x84, 0x24]);
				self.emit(((dst.0 * 8) as u32).to_ne_bytes());
			}
		}
	}

	fn emit<const N: usize>(&mut self, bytes: [u8; N]) {
		self.output.extend(bytes);
	}

	fn encode_relative_32(&mut self, position: usize, target: usize) {
		let target = (std::num::Wrapping(target) - std::num::Wrapping(position + 4)).0;

		*self.output.get_mut(position + 0).unwrap() = ((target >> 00) & 0xFF) as u8;
		*self.output.get_mut(position + 1).unwrap() = ((target >> 08) & 0xFF) as u8;
		*self.output.get_mut(position + 2).unwrap() = ((target >> 16) & 0xFF) as u8;
		*self.output.get_mut(position + 3).unwrap() = ((target >> 24) & 0xFF) as u8;
	}
}
