use std::{env, fs};

use linger::{interp, interpreter::interp_program, parser::parse_program, tokenizer::tokenize};

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
        Ok(t) => t,
        Err(e) => return println!("{e}"),
    };
    dbg!(&tokens);

    // let program = match parse_program(tokens.as_slice()) {
    //     Ok(p) => p,
    //     Err(e) => return println!("{e}"),
    // };

    // // dbg!(&program.main);

    // // let value = match interp(linger_file_content) {
    // //     Ok(v) => v,
    // //     Err(e) => {
    // //         println!("{e}");
    // //         return;
    // //     }
    // // };

    // let value = match interp_program(program) {
    //     Ok(v) => v,
    //     Err(e) => return println!("{e}"),
    // };

    // dbg!(value);
}
