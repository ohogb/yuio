use crate::hlir;

#[derive(Default)]
struct Scope {
	variables: std::collections::HashMap<String, (usize, hlir::ValueType)>,
}

pub struct Context {
	functions: std::collections::HashMap<String, usize>,
	function_count: usize,
	local_variables: Vec<hlir::ValueType>,
	stack: Vec<Scope>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			functions: std::collections::HashMap::new(),
			function_count: 0,
			local_variables: Vec::new(),
			stack: Vec::new(),
		}
	}

	pub fn define_function(&mut self, name: String) {
		let index = self.function_count;
		self.function_count += 1;

		self.functions.insert(name, index);
	}

	pub fn find_function(&self, name: &String) -> Option<usize> {
		self.functions.get(name).cloned()
	}

	pub fn push_scope(&mut self) {
		self.stack.push(Scope::default());
	}

	pub fn pop_scope(&mut self) {
		self.stack.pop();

		if self.stack.is_empty() {
			self.local_variables.clear();
		}
	}

	pub fn define_variable(&mut self, name: String, typ: hlir::ValueType) -> usize {
		let index = self.local_variables.len();
		self.local_variables.push(typ);

		self.stack
			.last_mut()
			.unwrap()
			.variables
			.insert(name, (index, typ));

		index
	}

	pub fn find_variable(&mut self, name: &String) -> Option<(usize, hlir::ValueType)> {
		for i in self.stack.iter().rev() {
			if let Some(x) = i.variables.get(name).cloned() {
				return Some(x);
			}
		}

		None
	}

	pub fn local_variables(&self) -> &Vec<hlir::ValueType> {
		&self.local_variables
	}
}
