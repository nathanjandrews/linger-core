use std::{env, fs};

use linger::{parser::parse_program, tokenizer::tokenize};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: linger <FILE>");
        return;
    }

    let linger_file_name = args[1].as_str();

    let linger_file_content = match fs::read_to_string(linger_file_name) {
        Ok(content) => content,
        Err(e) => {
            println!("error opening {linger_file_name}: {e}");
            return;
        }
    };

    let tokens = match tokenize(linger_file_content.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("tokenizer error: {}", e);
            return;
        }
    };
    // dbg!(&tokens);

    let program = match parse_program(tokens.as_slice()) {
        Ok(program) => program,
        Err(e) => {
            println!("parse error: {}", e);
            return;
        }
    };

    dbg!(program);
}
