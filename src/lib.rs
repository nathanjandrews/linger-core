use interpreter::interp_program;
use parser::parse_program;
use tokenizer::tokenize;

pub mod desugar;
pub mod error;
pub mod interpreter;
pub mod parser;
mod test;
pub mod tokenizer;

pub static KEYWORDS: &'static [&str] = &[
    "if", "else", "proc", "let", "true", "false", "return", "lam",
];

pub fn interp<'a>(s: String) -> Result<String, String> {
    let tokens = match tokenize(s.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.to_string()),
    };
    let program = match parse_program(tokens.as_slice()) {
        Ok(program) => program,
        Err(e) => return Err(e.to_string()),
    };
    return match interp_program(program) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e.to_string()),
    };
}
