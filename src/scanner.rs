use std::collections::HashMap;

use crate::err_report;
use crate::err_report::InterpreterError;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

/// Scanner for the lox language.
#[derive(Clone, Debug,)]
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
		while !this.at_end() {
			this.scan_token();
			this.start = this.current;
		}

		//pushing the EOF token cannot be done by scan_token()
		self.tokens.push(Token::new(EOF, "".to_string(),),);
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
			'%' => self.add_token(Mod,),
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
					while self.peek() != '\n' && !self.at_end() {
						self.eat();
					}
				} else if self.next_is('*',) {
					self.block_comment();
				} else {
					self.add_token(Slash,)
				}
			},
			' ' | '\r' | '\t' => (), // ignore whitespace, tab(indent)
			'\n' => self.line += 1,
			'"' => self.string(),
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

	/// in the book, this `fn` is originally implemented that
	/// ```rust
	/// fn eat(&mut self,) -> char {
	/// 	self.current += 1;`
	/// 	self.peek()
	/// }
	/// ```
	/// but this implementation will not scan some edge case correctly. Like:
	/// ```
	/// //files which starts with a comment
	/// var a = 1;
	/// ```
	fn eat(&mut self,) -> char {
		let c = self.peek();
		self.current += 1;
		c
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
		self.current += 1;
		while !(self.peek() == '"' || self.at_end()) {
			if self.peek() == '\n' {
				self.line += 1;
			}
			self.eat();
		}

		if self.at_end() {
			self.error(InterpreterError::new().occur(err_report::ErrorKind::UnterminatedString(
				self.src[self.start..].to_string(),
			),),);
			return;
		}

		self.current += 1;
		self.add_token(Str,);
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

	fn block_comment(&mut self,) {
		while !(self.next_is('*',) && self.next_is('/',)) {
			if self.next_is('\n',) {
				self.line += 1;
			} else if self.next_is('/',) && self.next_is('*',) {
				self.block_comment();
			} else if self.current >= self.src.len() {
				self.error(
					InterpreterError::new().occur(err_report::ErrorKind::UnterminatedComment,),
				);
			} else {
				self.eat();
			}
		}
	}

	fn add_token(&mut self, tt: TokenType,) {
		let lexeme = self.src[self.start..self.current].to_string();
		self.tokens.push(Token::new(tt, lexeme,),);
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
		assert_eq!(scanner.scan_tokens().len(), 1);
	}

	#[test]
	fn treat_whitespace() {
		let scanner = Scanner::new("\r\t ".to_string(),);
		assert_eq!(scanner.scan_tokens().len(), 1);
	}

	#[test]
	fn treat_newline() {
		let scanner = Scanner::new("\n\n".to_string(),);
		assert_eq!(scanner.scan_tokens().len(), 1,);
	}

	#[test]
	fn statements_test() {
		let scanner = Scanner::new("var a = 1;".to_string(),);
		let expect = vec![
			Token::new(Var, "var".to_string(),),
			Token::new(Identifier, "a".to_string(),),
			Token::new(Equal, "=".to_string(),),
			Token::new(Number, "1".to_string(),),
			Token::new(Semicolon, ";".to_string(),),
			Token::new(EOF, "".to_string(),),
		];
		assert_eq!(expect, scanner.scan_tokens(),);
	}

	#[test]
	fn multiline_test() {
		let scanner = Scanner::new("var a = 1;\nvar b = 2;\nprint \"hello world\";".to_string(),);
		let expect = vec![
			Token::new(Var, "var".to_string(),),
			Token::new(Identifier, "a".to_string(),),
			Token::new(Equal, "=".to_string(),),
			Token::new(Number, "1".to_string(),),
			Token::new(Semicolon, ";".to_string(),),
			Token::new(Var, "var".to_string(),),
			Token::new(Identifier, "b".to_string(),),
			Token::new(Equal, "=".to_string(),),
			Token::new(Number, "2".to_string(),),
			Token::new(Semicolon, ";".to_string(),),
			Token::new(Print, "print".to_string(),),
			Token::new(Str, "\"hello world\"".to_string(),),
			Token::new(Semicolon, ";".to_string(),),
			Token::new(EOF, "".to_string(),),
		];
		assert_eq!(expect, scanner.scan_tokens(),);
	}

	#[test]
	fn comment_test() {
		let scanner = Scanner::new("var a = 1; // this is a comment".to_string(),);
		let expect = vec![
			Token::new(Var, "var".to_string(),),
			Token::new(Identifier, "a".to_string(),),
			Token::new(Equal, "=".to_string(),),
			Token::new(Number, "1".to_string(),),
			Token::new(Semicolon, ";".to_string(),),
			Token::new(EOF, "".to_string(),),
		];
		assert_eq!(expect, scanner.scan_tokens(),);
	}
}
