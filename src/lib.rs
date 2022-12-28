pub mod error;
pub mod interpreter;
pub mod parser;
mod test;
pub mod tokenizer;

pub static KEYWORDS: &'static [&str] = &["if", "else", "proc", "let", "true", "false", "return"];
