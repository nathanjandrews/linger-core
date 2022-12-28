use std::io::{self, Write};

use interpreter::{interp_program, Value};
use parser::parse_program;
use tokenizer::tokenize;

pub mod error;
pub mod interpreter;
pub mod parser;
mod test;
pub mod tokenizer;

pub static KEYWORDS: &'static [&str] = &["if", "else", "proc", "let", "true", "false", "return"];

pub fn interp<'a>(s: String) -> Result<Value, String> {
  let tokens = match tokenize(s.as_str()) {
      Ok(tokens) => tokens,
      Err(e) => return Err(e.to_string()),
  };
  let program = match parse_program(tokens.as_slice()) {
      Ok(program) => program,
      Err(e) => return Err(e.to_string()),
  };
  return match interp_program(program) {
      Ok(value) => Ok(value),
      Err(e) => Err(e.to_string()),
  };
}

pub fn interp_to_string<'a>(s: String) -> Result<(Value, String), String> {
  let buffer = "".to_string();
  match io::stdout().write_all(buffer.as_bytes()) {
      Ok(_) => (),
      Err(e) => return Err(e.to_string()),
  };
  match interp(s) {
      Ok(v) => Ok((v, buffer)),
      Err(e) => Err(e),
  }
}