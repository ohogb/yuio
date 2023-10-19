#[derive(Debug)]
pub enum Operator {
	Add,
	Sub,
	Mul,
	Div,
	Assignment,
	Equal,
	NotEqual,
}

impl Operator {
	pub fn precedence(&self) -> u8 {
		match *self {
			Operator::Add => 3,
			Operator::Sub => 3,
			Operator::Mul => 4,
			Operator::Div => 4,
			Operator::Assignment => 1,
			Operator::Equal => 2,
			Operator::NotEqual => 2,
		}
	}
}
