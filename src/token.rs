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
	Mod,

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
	Assert,
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
			Mod => "%",
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
			Assert => "assert",
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
			And | Assert | Class | Else | False | For | Fun | If | Nil | Or | Print | Return
			| Super | This | True | Var | While => Some(format!("{self}"),),
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
			Mod => 11,
			Bang => 12,
			BangEqual => 13,
			Equal => 14,
			EqualEqual => 15,
			Greater => 16,
			GreaterEqual => 17,
			Less => 18,
			LessEqual => 19,
			Identifier => 20,
			Str => 21,
			Number => 22,
			And => 23,
			Assert => 24,
			Class => 25,
			Else => 26,
			False => 27,
			Fun => 28,
			For => 29,
			If => 30,
			Nil => 31,
			Or => 32,
			Print => 33,
			Return => 34,
			Super => 35,
			This => 36,
			True => 37,
			Var => 38,
			While => 39,
			EOF => 40,
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
			11 => Some(Mod,),
			12 => Some(Bang,),
			13 => Some(BangEqual,),
			14 => Some(Equal,),
			15 => Some(EqualEqual,),
			16 => Some(Greater,),
			17 => Some(GreaterEqual,),
			18 => Some(Less,),
			19 => Some(LessEqual,),
			20 => Some(Identifier,),
			21 => Some(Str,),
			22 => Some(Number,),
			23 => Some(And,),
			24 => Some(Assert,),
			25 => Some(Class,),
			26 => Some(Else,),
			27 => Some(False,),
			28 => Some(Fun,),
			29 => Some(For,),
			30 => Some(If,),
			31 => Some(Nil,),
			32 => Some(Or,),
			33 => Some(Print,),
			34 => Some(Return,),
			35 => Some(Super,),
			36 => Some(This,),
			37 => Some(True,),
			38 => Some(Var,),
			39 => Some(While,),
			40 => Some(EOF,),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq,)]
pub struct Token {
	token_type: TokenType,
	lexeme:     String,
	//line:       usize,
}

impl Token {
	pub fn new(token_type: TokenType, lexeme: String,) -> Self { Self { token_type, lexeme, } }
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter,) -> std::fmt::Result {
		let tt = format!("{:?}", self.token_type,);

		write!(f, "{tt} {}", self.lexeme)
	}
}
