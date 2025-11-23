use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    Strings(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

// read a file and return a string
pub fn read_file(filepath: &str) -> Result<String, std::io::Error> {
    // ? propagates error
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content)
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: Kind,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_kind: Kind, str_lexeme: String, line_num: u32) -> Self {
        Self {
            kind: token_kind,
            lexeme: str_lexeme,
            line: line_num,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Kind {
    BeginArray,     // [
    BeginObject,    // {
    EndArray,       // ]
    EndObject,      // }
    NameSeparator,  // :
    ValueSeparator, // ,
    Boolean(bool),  // true, false
    Null,           // null
    String(String), // string
    Number(f64),    // number
    EOF,
}

pub mod lexen {
    use super::*;

    #[derive(Debug)]
    pub struct Lexer {
        pub content: String,
        pub start: usize,
        pub current: usize,
        line: u32,
        tokens: Vec<Token>,
    }

    impl Lexer {
        pub fn new(input: String) -> Self {
            Self {
                content: input,
                start: 0,
                current: 0,
                line: 0,
                tokens: Vec::new(),
            }
        }

        pub fn lex(&mut self) -> Vec<Token> {
            while self.not_at_end() {
                self.start = self.current;
                self.start();
            }

            self.tokens
                .push(Token::new(Kind::EOF, "".to_string(), self.line));

            // return value
            return self.tokens.clone();
        }

        fn start(&mut self) {
            match self.advance() {
                None => {
                    if self.not_at_end() {
                        panic!("no char found.");
                    }
                }
                Some(xx) => {
                    println!("{xx}");
                    match xx {
                        '{' => self.tokens.push(Token {
                            kind: Kind::BeginObject,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        '[' => self.tokens.push(Token {
                            kind: Kind::BeginArray,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        '}' => self.tokens.push(Token {
                            kind: Kind::EndObject,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        ']' => self.tokens.push(Token {
                            kind: Kind::EndArray,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        ':' => self.tokens.push(Token {
                            kind: Kind::NameSeparator,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        ',' => self.tokens.push(Token {
                            kind: Kind::ValueSeparator,
                            lexeme: xx.to_string(),
                            line: self.line,
                        }),
                        '"' => self.read_string(),
                        '-' | '0'..='9' => self.read_number(), // numbers may be -ve
                        't' | 'f' | 'n' => self.read_literal(),
                        '\t' | ' ' => _ = self.advance(),
                        '\r' | '\n' => {
                            self.advance();
                            self.line += 1;
                        }
                        _ => {
                            self.dump();
                            panic!("unknown char={xx} found at {}", self.line);
                        }
                    }
                }
            }
        }

        fn dump(&self) {
            println!("Dumping. {} items", self.tokens.len());
            let mut idx = 0;
            while let Some(pat) = self.tokens.get(idx) {
                println!("{:?}", pat);
                idx += 1
            }
        }

        fn not_at_end(&self) -> bool {
            return self.current < self.content.len();
        }

        fn peek(&self) -> Option<char> {
            self.content[self.current..].chars().next()
        }

        fn previous(&self) -> Option<char> {
            self.content[self.current - 1..].chars().next()
        }

        fn advance(&mut self) -> Option<char> {
            // self.content.chars().nth();
            let ch: Option<char> = self.content[self.current..].chars().next();
            self.current += 1;
            ch
        }

        fn read_string(&mut self) {
            while let Some(pat) = self.peek() {
                self.advance();
                if pat == 'x' {
                    break;
                }
            }
            let s: String = (&self.content[self.start + 1..self.current - 1]).to_string();
            self.tokens.push(Token {
                kind: Kind::String(s),
                lexeme: (&self.content[self.start..self.current]).to_string(),
                line: self.line,
            });
        }

        fn read_number(&mut self) {
            let mut number = String::new();
            if let Some(prev) = self.previous() {
                number.push(prev);
            }
            // integer part
            while let Some(pat) = self.peek() {
                if pat.is_digit(10) {
                    number.push(pat);
                    self.advance();
                } else if pat == '.' {
                    break;
                } else {
                    panic!("unknown char {pat} found at {}", self.line);
                }
            }

            // fractional part
            let mut exp: bool = false;
            if let Some('.') = self.peek() {
                self.advance(); // consume .
                while let Some(pat) = self.peek() {
                    if pat.is_digit(10) {
                        number.push(pat);
                        self.advance();
                    } else if pat == 'e' || pat == 'E' {
                        exp = true;
                        break;
                    } else {
                        panic!("unknown char {pat} found at {}", self.line);
                    }
                }
            }

            // exponent part
            if exp {
                if let Some('-' | '+') = self.peek() {
                    self.advance();
                }
                while let Some(pat) = self.peek() {
                    if pat.is_whitespace() {
                        if pat == '\n' || pat == '\r' {
                            self.line += 1;
                        }
                        self.advance();
                        break;
                    }
                    if pat.is_digit(10) {
                        number.push(pat);
                        self.advance();
                    } else {
                        panic!("unknown char {pat} found at {}", self.line);
                    }
                }
            }

            if number.ends_with("+") || number.ends_with("-") {
                panic!("unknown sequence {number} found at {}", self.line);
            }

            let value = match number.parse::<f64>() {
                Ok(f) => f,
                Err(err) => {
                    panic!(
                        "unable to parse {} @ ln{} with error {}",
                        number, self.line, err
                    );
                }
            };

            self.tokens.push(Token {
                kind: Kind::Number(value),
                lexeme: number,
                line: self.line,
            });
        }

        fn read_literal(&mut self) {
            let mut s: String = String::new();
            while let Some(ch @ 'a'..='z') = self.peek() {
                self.advance();
                s.push(ch);
            }

            let kd = match s.as_str() {
                "true" => Kind::Boolean(true),
                "false" => Kind::Boolean(false),
                "null" => Kind::Null,
                _ => panic!("unknown literal `{s}` @ line{}", self.line),
            };

            self.tokens.push(Token {
                kind: kd,
                lexeme: s,
                line: self.line,
            });
        }

        fn is_digit(c: char) -> bool {
            return '0' <= c && c <= '9';
        }
    }
}
