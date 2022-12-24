use std::{env, fs};

use tokenizer::tokenize::tokenize;

mod tokenizer;

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

    let tokens = match tokenize(linger_file_content) {
        Ok(tokens) => tokens,
        Err(_) => {
            println!("tokenizer error");
            return;
        }
    };

    dbg!(tokens);
}
