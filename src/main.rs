use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
mod evaluator;
mod native_functions;
mod parser;
mod tokenizer;

use parser::parser::Parser;
use tokenizer::tokenize::Tokenizer;

use crate::evaluator::evaluate::Eval;
use crate::evaluator::virtual_machine::VirtualMachine;

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
                match token {
                    _ if token.is_ignorable() => {
                        // Do nothing, skip this token
                    }
                    _ if token.is_error() => {
                        eprintln!("{}", token);
                    }
                    _ => println!("{}", token),
                }
            }
            if tokenizer.has_errors() {
                exit(65);
            } else {
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut parser = Parser::new(file_contents);
            let tree = parser.parse();
            writeln!(io::stdout(), "{}", tree).unwrap();
        }
        "evaluate" | "run" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut parser = Parser::new(file_contents);
            let tree = parser.parse();
            let mut vm = VirtualMachine::new();
            tree.nodes
                .iter()
                .map(|n| n.evaluate(&mut vm))
                .filter(|n| !n.is_nothing())
                .for_each(|n| println!("{}", n));
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
