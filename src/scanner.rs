use std::collections::HashMap;

use crate::err_report;
use crate::err_report::InterpreterError;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

/// Scanner for the lox language.
#[derive(Clone,)]
pub struct Scanner {
	had_err:  InterpreterError,
	src:      String,
	tokens:   Vec<Token,>,
	start:    usize,
	current:  usize,
	line:     usize,
	keywords: HashMap<String, TokenType,>,
}

impl Scanner {
	pub fn new(src: String,) -> Self {
		let mut keywords = HashMap::new();
		for tt in And..=While {
			keywords.insert(tt.keywords().unwrap(), tt,);
		}

		Self {
			had_err: InterpreterError::new(),
			src,
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
			keywords,
		}
	}

	pub fn scan_tokens(mut self,) -> Vec<Token,> {
		let this = &mut self;
		while this.at_end() {
			this.start = this.current;
			this.scan_token();
		}

		this.tokens.push(Token::new(EOF, "".to_string(), this.line,),);
		self.tokens
	}

	fn scan_token(&mut self,) {
		match self.eat() {
			'(' => self.add_token(LeftParen,),
			')' => self.add_token(RightParen,),
			'{' => self.add_token(LeftBrace,),
			'}' => self.add_token(RightBrace,),
			',' => self.add_token(Comma,),
			'.' => self.add_token(Dot,),
			'-' => self.add_token(Minus,),
			'+' => self.add_token(Plus,),
			';' => self.add_token(Semicolon,),
			'*' => self.add_token(Star,),
			'!' => {
				let tk = if self.next_is('=',) { BangEqual } else { Bang };
				self.add_token(tk,)
			},
			'=' => {
				let tk = if self.next_is('=',) { EqualEqual } else { Equal };
				self.add_token(tk,)
			},
			'<' => {
				let tk = if self.next_is('=',) { LessEqual } else { Less };
				self.add_token(tk,)
			},
			'>' => {
				let tk = if self.next_is('=',) { GreaterEqual } else { Greater };
				self.add_token(tk,)
			},
			'/' => {
				if self.next_is('/',) {
					while !(self.peek() != '\n' || self.at_end()) {
						self.eat();
					}
				} else {
					self.add_token(Slash,)
				}
			},
			' ' | '\r' | '\t' => (), // ignore whitespace
			'\n' => self.line += 1,
			'"' => self.string(),
			'o' => {
				if self.next_is('r',) {
					self.add_token(Or,)
				}
			},
			c => {
				if self.is_digit(c,) {
					self.number();
				} else if self.is_alpha(c,) {
					self.identifier()
				} else {
					self.error(
						InterpreterError::new()
							.occur(err_report::ErrorKind::UnexpectedCharacter(c,),),
					)
				}
			},
		}
	}

	fn eat(&mut self,) -> char {
		self.current += 1;
		self.peek()
	}

	// p:
	/// for performance matter, we don't use `self.peek() == '\0'`
	fn at_end(&self,) -> bool { self.current >= self.src.len() }

	fn next_is(&mut self, expected: char,) -> bool {
		if self.peek() != expected {
			return false;
		}

		self.current += 1;
		true
	}

	fn peek(&self,) -> char { self.src.chars().nth(self.current,).unwrap_or('\0',) }

	fn peek2(&self,) -> char { self.src.chars().nth(self.current + 1,).unwrap_or('\0',) }

	fn is_alphanumeric(&self, c: char,) -> bool { self.is_alpha(c,) || self.is_digit(c,) }

	fn is_digit(&self, c: char,) -> bool { '0' <= c && c <= '9' }

	fn is_alpha(&self, c: char,) -> bool {
		('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
	}

	fn string(&mut self,) {
		while !(self.peek() != '"' || self.at_end()) {
			if self.peek() == '\n' {
				self.line += 1;
			}
			self.eat();
		}

		if self.at_end() {
			self.error(InterpreterError::new().occur(err_report::ErrorKind::UnterminatedString(
				// d: `self.start+1` trims the leading `"`
				self.src[self.start + 1..=self.current].to_string(),
			),),);
			return;
		}

		self.eat(); // eat the closing `"`
		self.add_token(Str,)
	}

	fn number(&mut self,) {
		while self.is_digit(self.peek(),) {
			self.eat();
		}

		if self.peek() == '.' && self.is_digit(self.peek2(),) {
			self.eat();
			while self.is_digit(self.peek(),) {
				self.eat();
			}
		}

		self.add_token(Number,)
	}

	fn identifier(&mut self,) {
		while self.is_alphanumeric(self.peek(),) {
			self.eat();
		}

		let txt = &self.src[self.start..self.current];
		// if `txt` does not match any reserved keywords, it will be `Identifier`
		let tt = self.keywords.get(txt,).unwrap_or(&Identifier,);
		self.add_token(tt.clone(),);
	}

	fn add_token(&mut self, tt: TokenType,) {
		let is_str = if let Str = tt { 1 } else { 0 };
		let lexeme = self.src[self.start + is_str..self.current - is_str].to_string();
		self.tokens.push(Token::new(tt, lexeme, self.line,),);
	}

	fn error(&mut self, e: InterpreterError,) {
		self.had_err = e.clone();
		err_report::error(self.line, e,)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn esc_code() {
		let mut scanner = Scanner::new("".to_string(),);
		assert_eq!(scanner.eat(), '\0',);
		println!("this is \\0 [\0]")
	}
}
