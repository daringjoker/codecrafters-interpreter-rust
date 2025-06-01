use std::env;
use std::fs;
use std::io::{self, Write};
mod tokenizer;

use tokenizer::tokenize::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut tokenizer = Tokenizer::new(file_contents);
            let tokens = tokenizer.tokenize();
            for token in tokens {
                if token.is_ignorable() {
                    continue; // Skip whitespace and comments
                }
                println!("{}", token);
            }
        }
        "tokenize-string" => {
            if args.len() < 3 {
                writeln!(io::stderr(), "Usage: {} tokenize-string <string>", args[0]).unwrap();
                return;
            }
            let input_string = &args[2];
            let mut tokenizer = Tokenizer::new(input_string.clone());
            let tokens = tokenizer.tokenize();
            for token in tokens {
                if token.is_ignorable() {
                    continue; // Skip whitespace and comments
                }
                println!("{}", token);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
