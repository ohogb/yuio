#[derive(Debug, PartialEq)]
pub enum Token {
	Identifier(String),
	Number(String),

	If,
	Fn,
	Return,
	Let,

	OpeningParen,
	ClosingParen,
	OpeningCurly,
	ClosingCurly,

	Colon,
	SemiColon,
	Comma,

	Plus,
	Minus,
	Star,
	Slash,
	Equals,
	ExclamationMark,
	QuestionMark,
}

impl Token {
	pub fn precedence(&self) -> u8 {
		match *self {
			Token::Plus => 2,
			Token::Minus => 2,
			Token::Star => 3,
			Token::Slash => 3,
			Token::Equals => 1,
			_ => unreachable!(),
		}
	}
}
