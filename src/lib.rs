pub mod tokenizer;
pub mod parser;
pub mod interpreter;
pub mod error;
mod test;

pub static KEYWORDS: &'static [&str] = &["if", "else", "proc", "let", "true", "false", "return"];
