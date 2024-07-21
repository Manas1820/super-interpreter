use interpreter_starter_rust::parser::Parser;
use interpreter_starter_rust::scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut exit_code = ExitCode::SUCCESS;

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return exit_code;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                if !scanner.errors.is_empty() {
                    exit_code = ExitCode::from(65);
                }

                for error in scanner.errors {
                    eprintln!("{}", error);
                }

                for token in scanner.tokens {
                    println!("{}", token);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                // Scan the tokens
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                // Parse the tokens
                let mut parser = Parser::new(scanner.tokens);

                let parsed_result = parser.parse();

                if !scanner.errors.is_empty() {
                    exit_code = ExitCode::from(65);

                    for error in &scanner.errors {
                        eprintln!("{}", error);
                    }
                } else if !parser.errors.is_empty() {
                    exit_code = ExitCode::from(65);

                    for error in &parser.errors {
                        eprintln!("{}", error);
                    }
                } else {
                    for paresd in parsed_result {
                        println!("{}", paresd);
                    }
                }
            } else {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                return exit_code;
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return exit_code;
        }
    }

    exit_code
}
