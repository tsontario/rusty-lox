use std::env;
use std::env::args;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Lexeme {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    EOF,
}

static TOKENS: LazyLock<HashMap<Lexeme, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (Lexeme::LEFT_PAREN, "("),
        (Lexeme::RIGHT_PAREN, ")"),
        (Lexeme::LEFT_BRACE, "{"),
        (Lexeme::RIGHT_BRACE, "}"),
        (Lexeme::EOF, ""),
    ])
});

impl Lexeme {
    fn parse(c: &char) -> Option<Lexeme> {
        match c {
            '(' => Some(Lexeme::LEFT_PAREN),
            ')' => Some(Lexeme::RIGHT_PAREN),
            '{' => Some(Lexeme::LEFT_BRACE),
            '}' => Some(Lexeme::RIGHT_BRACE),
            _ => None
        }
    }
}


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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let mut lexemes = Vec::new();
                file_contents.chars().for_each(|c| {
                    if let Some(lexeme) = Lexeme::parse(&c) {
                        lexemes.push(lexeme);
                    } else {
                        // ignore for now (whitespace, etc.)
                    }
                });
                lexemes.push(Lexeme::EOF);
                lexemes.iter().for_each(|l| println!("{:?} {} null", l, TOKENS.get(l).unwrap()));
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}