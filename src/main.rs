#![feature(step_trait)]

mod err_report;
mod scanner;
mod token;

use scanner::Scanner;
use std::io;

fn run_file(path: String,) -> io::Result<(),> { run(std::fs::read_to_string(path,)?,) }

fn run_prompt() -> io::Result<(),> {
	let input = std::io::stdin();
	loop {
		let mut line = String::new();
		println!("> ");
		if input.read_line(&mut line,)? == 0 {
			break;
		}
		let _ = run(line,);
		//		line.clear();
	}
	Ok((),)
}

fn run(src: String,) -> io::Result<(),> {
	let scan = Scanner::new(src,);
	let tokens = scan.scan_tokens();

	// d: currently, just print the tokens
	for token in tokens {
		println!("{token:?}",);
	}
	Ok((),)
}

fn main() -> io::Result<(),> {
	let mut args = std::env::args();
	if args.len() > 1 {
		println!("Usage: lox [script]");
		Ok((),)
	} else if args.len() == 1 {
		run_file(args.next().expect("🫠main: cmdline arg not found",),)
	} else {
		run_prompt()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
