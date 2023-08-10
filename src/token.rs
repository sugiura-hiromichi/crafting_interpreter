use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash,)]
#[allow(dead_code)]
pub enum TokenType {
	// single character tokens
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// one or two character tokens
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// literals
	Identifier,
	Str,
	Number,

	// keywords
	And,
	Class,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Var,
	While,

	EOF,
}

impl Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		use self::TokenType::*;
		let tt = match self {
			LeftParen => "(",
			RightParen => ")",
			LeftBrace => "{",
			RightBrace => "}",
			Comma => ",",
			Dot => ".",
			Minus => "-",
			Plus => "+",
			Semicolon => ";",
			Slash => "/",
			Star => "*",
			Bang => "!",
			BangEqual => "!=",
			Equal => "=",
			EqualEqual => "==",
			Greater => ">",
			GreaterEqual => ">=",
			Less => "<",
			LessEqual => "<=",
			Identifier => "token_identifier",
			Str => "token_string",
			Number => "token_number",
			And => "and",
			Class => "class",
			Else => "else",
			False => "false",
			Fun => "fun",
			For => "for",
			If => "if",
			Nil => "nil",
			Or => "or",
			Print => "print",
			Return => "return",
			Super => "super",
			This => "this",
			True => "true",
			Var => "var",
			While => "while",
			EOF => "EOF",
		};
		write!(f, "{tt}")
	}
}

impl std::iter::Step for TokenType {
	fn steps_between(start: &Self, end: &Self,) -> Option<usize,> {
		let (start, end,) = (start.clike_enum(), end.clike_enum(),);
		if start <= end {
			Some(end - start,)
		} else {
			None
		}
	}

	fn forward_checked(start: Self, count: usize,) -> Option<Self,> {
		let dest = start.clike_enum() + count;
		TokenType::rustize(dest,)
	}

	fn backward_checked(start: Self, count: usize,) -> Option<Self,> {
		let strt = start.clike_enum();
		if strt < count {
			None
		} else {
			TokenType::rustize(strt - count,)
		}
	}
}

impl TokenType {
	pub fn keywords(&self,) -> Option<String,> {
		use self::TokenType::*;
		match self {
			And | Class | Else | False | For | Fun | If | Nil | Or | Print | Return | Super
			| This | True | Var | While => Some(format!("{self}"),),
			_ => None,
		}
	}

	pub(super) fn clike_enum(&self,) -> usize {
		use self::TokenType::*;
		match self {
			LeftParen => 0,
			RightParen => 1,
			LeftBrace => 2,
			RightBrace => 3,
			Comma => 4,
			Dot => 5,
			Minus => 6,
			Plus => 7,
			Semicolon => 8,
			Slash => 9,
			Star => 10,
			Bang => 11,
			BangEqual => 12,
			Equal => 13,
			EqualEqual => 14,
			Greater => 15,
			GreaterEqual => 16,
			Less => 17,
			LessEqual => 18,
			Identifier => 19,
			Str => 20,
			Number => 21,
			And => 22,
			Class => 23,
			Else => 24,
			False => 25,
			Fun => 26,
			For => 27,
			If => 28,
			Nil => 29,
			Or => 30,
			Print => 31,
			Return => 32,
			Super => 33,
			This => 34,
			True => 35,
			Var => 36,
			While => 37,
			EOF => 38,
		}
	}

	pub(super) fn rustize(clike_enum: usize,) -> Option<Self,> {
		use self::TokenType::*;
		match clike_enum {
			0 => Some(LeftParen,),
			1 => Some(RightParen,),
			2 => Some(LeftBrace,),
			3 => Some(RightBrace,),
			4 => Some(Comma,),
			5 => Some(Dot,),
			6 => Some(Minus,),
			7 => Some(Plus,),
			8 => Some(Semicolon,),
			9 => Some(Slash,),
			10 => Some(Star,),
			11 => Some(Bang,),
			12 => Some(BangEqual,),
			13 => Some(Equal,),
			14 => Some(EqualEqual,),
			15 => Some(Greater,),
			16 => Some(GreaterEqual,),
			17 => Some(Less,),
			18 => Some(LessEqual,),
			19 => Some(Identifier,),
			20 => Some(Str,),
			21 => Some(Number,),
			22 => Some(And,),
			23 => Some(Class,),
			24 => Some(Else,),
			25 => Some(False,),
			26 => Some(Fun,),
			27 => Some(For,),
			28 => Some(If,),
			29 => Some(Nil,),
			30 => Some(Or,),
			31 => Some(Print,),
			32 => Some(Return,),
			33 => Some(Super,),
			34 => Some(This,),
			35 => Some(True,),
			36 => Some(Var,),
			37 => Some(While,),
			38 => Some(EOF,),
			_ => None,
		}
	}
}

#[derive(Debug, Clone,)]
pub struct Token {
	token_type: TokenType,
	lexeme:     String,
	line:       usize,
}

impl Token {
	pub fn new(token_type: TokenType, lexeme: String, line: usize,) -> Self {
		Self { token_type, lexeme, line, }
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter,) -> std::fmt::Result {
		let tt = format!("{:?}", self.token_type,);

		write!(f, "{tt} {}", self.lexeme)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
