//! Error reporting module

#[derive(Debug, Clone,)]
pub struct InterpreterError {
	kind:    ErrorKind,
	had_err: bool,
}
impl std::error::Error for InterpreterError {}

impl std::fmt::Display for InterpreterError {
	fn fmt(&self, f: &mut std::fmt::Formatter,) -> std::fmt::Result { write!(f, "{}", self.kind,) }
}

impl InterpreterError {
	pub fn new() -> Self { Self { kind: ErrorKind::Unknown, had_err: false, } }

	pub fn occur(mut self, e: ErrorKind,) -> Self {
		self.had_err = true;
		self.kind = e;
		self
	}
}

#[derive(Debug, Clone,)]
pub enum ErrorKind {
	UnexpectedCharacter(char,),
	UnterminatedString(String,),
	UnterminatedComment,
	Unknown,
	//NaE,  Not an Error
}
impl std::fmt::Display for ErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		match self {
			ErrorKind::UnexpectedCharacter(c,) => write!(f, "Unexpected character: {c}",),
			ErrorKind::UnterminatedString(s,) => write!(f, "Unterminated string: {s}",),
			ErrorKind::UnterminatedComment => write!(f, "Unterminated comment",),
			ErrorKind::Unknown => write!(f, "Unknown error",),
		}
	}
}

pub fn error(line: usize, err: impl std::error::Error,) { report(line, "".to_string(), err,) }

pub fn report(line: usize, place: String, err: impl std::error::Error,) {
	eprintln!("\u{ea87} Error at line {line} where {place} \u{ea87}\n{err}",);
}
