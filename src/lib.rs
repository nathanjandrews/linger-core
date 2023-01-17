use std::{fs::File, io::Write, path::Path};

use interpreter::interp_program;
use parser::parse_program;
use tokenizer::tokenize;

mod desugar;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod tokenizer;


pub struct Writer<'a> {
    w: Box<dyn Write + 'a>,
}

impl<'a> Writer<'a> {
    pub fn new(w: Box<dyn Write + 'a>) -> Self { Self { w } }
}

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

    let writer = &mut Writer {
        w: Box::new(std::io::stdout()),
    };

    return match interp_program(program, writer) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e.to_string()),
    };
}

pub fn interp_to_file<'a>(s: String, path: &Path) -> Result<String, String> {
    let tokens = match tokenize(s.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.to_string()),
    };
    let program = match parse_program(tokens.as_slice()) {
        Ok(program) => program,
        Err(e) => return Err(e.to_string()),
    };

    let file = match File::create(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let writer = &mut Writer { w: Box::new(file) };

    return match interp_program(program, writer) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e.to_string()),
    };
}

pub fn interp_to_buffer<'a>(s: String, buf: &mut Vec<u8>) -> Result<String, String> {
    let tokens = match tokenize(s.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.to_string()),
    };
    let program = match parse_program(tokens.as_slice()) {
        Ok(program) => program,
        Err(e) => return Err(e.to_string()),
    };

    let writer = &mut Writer { w: Box::new(buf) };

    return match interp_program(program, writer) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e.to_string()),
    };
}
