use interpreter::interp_program;
use parser::parse_program;
use tokenizer::tokenize;

mod desugar;
pub mod environment;
mod error;
pub mod interpreter;
pub mod parser;
pub mod tokenizer;

/// Executes a linger program. On success, this program returns the return value of the main
/// procedure as a String. If there is an error in any step of the program (tokenization, parsing,
/// or interpreting), this function will return that error as a [String].
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
