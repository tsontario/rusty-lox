use std::env;
use std::env::args;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use std::sync::LazyLock;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Lexeme {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    STAR,
    DOT,
    COMMA,
    PLUS,
    MINUS,
    SEMICOLON,
    SLASH,
    EOF,
}

static TOKENS: LazyLock<HashMap<Lexeme, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (Lexeme::LEFT_PAREN, "("),
        (Lexeme::RIGHT_PAREN, ")"),
        (Lexeme::LEFT_BRACE, "{"),
        (Lexeme::RIGHT_BRACE, "}"),
        (Lexeme::STAR, "*"),
        (Lexeme::DOT, "."),
        (Lexeme::COMMA, ","),
        (Lexeme::PLUS, "+"),
        (Lexeme::MINUS, "-"),
        (Lexeme::SEMICOLON, ";"),
        (Lexeme::SLASH, "/"),
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
            '*' => Some(Lexeme::STAR),
            '.' => Some(Lexeme::DOT),
            ',' => Some(Lexeme::COMMA),
            '+' => Some(Lexeme::PLUS),
            '-' => Some(Lexeme::MINUS),
            ';' => Some(Lexeme::SEMICOLON),
            '/' => Some(Lexeme::SLASH),
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

            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);
            let mut lexemes = Vec::new();
            let mut has_errors = false;

            // Uncomment this block to pass the first stage
            for (line_no, line) in reader.lines().enumerate() {
                line.unwrap().chars().for_each(|c| {
                    if let Some(lexeme) = Lexeme::parse(&c) {
                        lexemes.push(lexeme);
                    } else {
                        has_errors = true;
                        eprintln!("[line 1] Error: Unexpected character: {}", c);
                    }
                });
            }
            lexemes.push(Lexeme::EOF);
            lexemes.iter().for_each(|l| println!("{:?} {} null", l, TOKENS.get(l).unwrap()));

            if has_errors {
                exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}