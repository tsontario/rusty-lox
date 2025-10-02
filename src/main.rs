use std::env;
use std::env::args;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::collections::HashMap;
use std::fs::File;
use std::ops::Index;
use std::process::exit;
use std::sync::LazyLock;
use anyhow::Result;
use crate::TokenType::PLUS;

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    literal: String,
    text: String, // TODO probably need a different struct for this
    line: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenType {
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
    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    EOF,
    ERROR
}

static TOKENS: LazyLock<HashMap<TokenType, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (TokenType::LEFT_PAREN, "("),
        (TokenType::RIGHT_PAREN, ")"),
        (TokenType::LEFT_BRACE, "{"),
        (TokenType::RIGHT_BRACE, "}"),
        (TokenType::STAR, "*"),
        (TokenType::DOT, "."),
        (TokenType::COMMA, ","),
        (TokenType::PLUS, "+"),
        (TokenType::MINUS, "-"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::SLASH, "/"),
        (TokenType::EQUAL, "="),
        (TokenType::EQUAL_EQUAL, "=="),
        (TokenType::BANG, "!"),
        (TokenType::BANG_EQUAL, "!="),
        (TokenType::LESS, "<"),
        (TokenType::LESS_EQUAL, "<="),
        (TokenType::GREATER, ">"),
        (TokenType::GREATER_EQUAL, ">="),
        (TokenType::EOF, ""),
    ])
});

impl TokenType {
    fn parse(c: &str) -> Option<TokenType> {
        match c {
            "(" => Some(TokenType::LEFT_PAREN),
            ")" => Some(TokenType::RIGHT_PAREN),
            "{" => Some(TokenType::LEFT_BRACE),
            "}" => Some(TokenType::RIGHT_BRACE),
            "*" => Some(TokenType::STAR),
            "." => Some(TokenType::DOT),
            "," => Some(TokenType::COMMA),
            "+" => Some(TokenType::PLUS),
            "-" => Some(TokenType::MINUS),
            ";" => Some(TokenType::SEMICOLON),
            "/" => Some(TokenType::SLASH),
            "=" => Some(TokenType::EQUAL),
            "==" => Some(TokenType::EQUAL_EQUAL),
            "!" => Some(TokenType::BANG),
            "!=" => Some(TokenType::BANG_EQUAL),
            "<" => Some(TokenType::LESS),
            "<=" => Some(TokenType::LESS_EQUAL),
            ">" => Some(TokenType::GREATER),
            ">=" => Some(TokenType::GREATER_EQUAL),
            "\n" | "\r" | "\r\n" => None,
            "" => Some(TokenType::EOF),
            _ => Some(TokenType::ERROR)
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_errors: bool
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_errors: false,
        }
    }

    fn scan_tokens(&mut self) -> () {
        while !self.eof() {
            self.start = self.current;
            let c = self.advance();
            if let Some(lexeme) = TokenType::parse(c.to_string().as_str()) {
                match lexeme {
                    TokenType::BANG => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::BANG_EQUAL);
                        } else {
                            self.add_token(lexeme);
                        }
                    }
                    TokenType::EQUAL => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::EQUAL_EQUAL);
                        } else {
                            self.add_token(lexeme);
                        }
                    }
                    TokenType::GREATER => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::GREATER_EQUAL);
                        } else {
                            self.add_token(lexeme);
                        }
                    }
                    TokenType::LESS => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::LESS_EQUAL);
                        } else {
                            self.add_token(lexeme);
                        }
                    }
                    _ => self.add_token(lexeme)
                }
            }
        }
        self.add_token(TokenType::EOF);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current-1).unwrap()
    }

    fn peek(&self) -> char {
        if self.eof() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) -> () {
        if token_type == TokenType::ERROR {
            self.has_errors = true;
        }

        let token = if token_type == TokenType::EOF {
            Token {
                token_type: token_type,
                text: String::from(""),
                literal: String::from(""),
                line: 1,
            }
        } else {
            let text = self.source.chars().skip(self.start).take(self.current - self.start).collect();
            Token {
                token_type: token_type,
                text: text,
                literal: String::from(""),
                line: 1,
            }
        };
        self.tokens.push(token);
    }

    fn is_compound_token(&mut self, c: char) -> bool {
        if self.eof() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() == c {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn eof(&self) -> bool {
        self.current == self.source.len()
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

            let mut has_errors = false;
            let mut scanner = Scanner::new(fs::read_to_string(filename).unwrap());
            scanner.scan_tokens();

            // TODO FIX THIS
            let (errors, tokens): (Vec<&Token>, Vec<&Token>) = scanner.tokens.iter().partition(|t| t.token_type == TokenType::ERROR);
            errors.iter().for_each(|e| eprintln!("[line 1] Error: Unexpected character: {}", e.text));
            tokens.iter().for_each(|l| {
                let text = l.text.as_str();
                println!("{:?} {} null", TokenType::parse(text).unwrap(), TOKENS.get(&l.token_type).unwrap());
            });

            if scanner.has_errors {
               exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}