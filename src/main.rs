use std::string::String;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::sync::LazyLock;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};


#[derive(Debug, Clone)]
enum Literal {
    String(String),
    NULL
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::NULL => write!(f, "null"),
        }
    }
}

enum ErrorType {
    UnexpectedCharacter(String),
    UnterminatedString(String)
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::UnexpectedCharacter(c) => write!(f, "Unexpected character: {}", c),
            ErrorType::UnterminatedString(s) => write!(f, "Unterminated string."),
        }
    }
}

struct Error {
    error_type: ErrorType,
    line: usize,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    literal: Option<Literal>,
    text: String, // TODO probably need a different struct for this
    line: usize,
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
    LINE_BREAK,
    ERROR,
    STRING
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
        (TokenType::LINE_BREAK, ""),
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
            "\r" | "\t" | " " => None,
            "\n" => Some(TokenType::LINE_BREAK),
            "\"" => Some(TokenType::STRING),
            "" => Some(TokenType::EOF),
            _ => Some(TokenType::ERROR)
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<Error>,
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
            errors: Vec::new(),
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
                            self.add_token(TokenType::BANG_EQUAL, None);
                        } else {
                            self.add_token(lexeme, None);
                        }
                    }
                    TokenType::EQUAL => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::EQUAL_EQUAL, None);
                        } else {
                            self.add_token(lexeme, None);
                        }
                    }
                    TokenType::ERROR => {
                        let unexpected_char = c.to_string();
                        self.add_error(ErrorType::UnexpectedCharacter(unexpected_char));
                    }
                    TokenType::GREATER => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::GREATER_EQUAL, None);
                        } else {
                            self.add_token(lexeme, None);
                        }
                    }
                    TokenType::LESS => {
                        if self.is_compound_token('=') {
                            self.add_token(TokenType::LESS_EQUAL, None);
                        } else {
                            self.add_token(lexeme, None);
                        }
                    }
                    TokenType::LINE_BREAK => {
                        self.line += 1;
                    }
                    TokenType::SLASH => {
                        if self.is_compound_token('/') {
                            while !self.eof() && self.peek() != "\n" {
                                self.current += 1; // Ignore comments
                            }
                        } else {
                            self.add_token(lexeme, None);
                        }
                    }
                    TokenType::STRING => {
                        while !self.eof() && self.peek() != "\"" {
                            self.current += 1;
                        }
                        if !self.eof() {
                            self.current += 1;
                            self.add_token(lexeme, Some(Literal::String(self.substr(self.start + 1, self.current - 1))));
                        } else {
                            self.add_error(ErrorType::UnterminatedString(self.substr(self.start, self.current)));
                        }
                    }
                    _ => self.add_token(lexeme, None)
                }
            }
        }
        self.add_token(TokenType::EOF, None);
    }

    fn substr(&self, start: usize, end: usize) -> String {
        self.source.graphemes(true).skip(start).take(end - start).collect()
    }
    fn advance(&mut self) -> &str {
        self.current += 1;
        self.source.graphemes(true).nth(self.current-1).unwrap()
    }

    fn peek(&self) -> &str {
        self.source.graphemes(true).nth(self.current).unwrap()
    }

    fn add_error(&mut self, error_type: ErrorType) -> () {
        self.has_errors = true;
        let error = Error {
            error_type,
            line: self.line
        };
        self.errors.push(error);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) -> () {
        let token = if token_type == TokenType::EOF {
            Token {
                token_type: token_type,
                text: String::from(""),
                literal,
                line: 1,
            }
        } else {
            let text = self.source.graphemes(true).skip(self.start).take(self.current - self.start).collect();
            Token {
                token_type: token_type,
                text: text,
                literal: literal,
                line: 1,
            }
        };
        self.tokens.push(token);
    }

    fn is_compound_token(&mut self, c: char) -> bool {
        if self.eof() {
            return false;
        }
        if self.source.graphemes(true).nth(self.current).unwrap() == c.to_string().as_str() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn eof(&self) -> bool {
        self.current == self.source.chars().count()
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

            scanner.errors.iter().for_each(|e| eprintln!("[line {}] Error: {}", e.line, e.error_type));
            scanner.tokens.iter().for_each(|l| {
                println!("{:?} {} {}", l.token_type, l.text.as_str(), l.literal.clone().unwrap_or(Literal::NULL));;
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