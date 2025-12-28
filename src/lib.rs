use std::collections::HashMap;
use std::fmt;
use std::fs;

pub mod parsen;

pub fn read_file(filepath: &str) -> Result<String, std::io::Error> {
    // ? propagates error
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content)
}

#[derive(Clone)]
pub struct Token {
    kind: Kind,
    line: u32,
    // lexeme: String, future use: cool to see what we actually consumed.
}

impl Token {
    pub fn get_line(&self) -> u32 {
        self.line
    }

    pub fn get_kind(&self) -> Kind {
        self.kind.clone()
    }
}

impl fmt::Debug for Token {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Token: (kind{:?}, line{})", self.kind, self.line)
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl Kind {
    pub fn to_json(self) -> parsen::Json {
        match self {
            Kind::Boolean(b) => parsen::Json::Boolean(b),
            Kind::String(s) => parsen::Json::Strings(s),
            Kind::Number(f) => parsen::Json::Number(f),
            Kind::Null => parsen::Json::Null,
            _ => {
                panic!("");
            }
        }
    }
}

//
// impl fmt::Debug for Kind {
// // add code here
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// // Use `self.number` to refer to each positional data point.
// write!(f, "{:?}", self)
// }
// }

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

        fn start(&mut self) {
            match self.advance() {
                None => {
                    if self.not_at_end() {
                        self.dump();
                        panic!("no char found.");
                    }
                }
                Some(xx) => {
                    // self.dump();
                    match xx {
                        '{' => self.tokens.push(Token {
                            kind: Kind::BeginObject,
                            line: self.line,
                        }),
                        '[' => self.tokens.push(Token {
                            kind: Kind::BeginArray,
                            line: self.line,
                        }),
                        '}' => self.tokens.push(Token {
                            kind: Kind::EndObject,
                            line: self.line,
                        }),
                        ']' => self.tokens.push(Token {
                            kind: Kind::EndArray,
                            line: self.line,
                        }),
                        ':' => self.tokens.push(Token {
                            kind: Kind::NameSeparator,
                            line: self.line,
                        }),
                        ',' => self.tokens.push(Token {
                            kind: Kind::ValueSeparator,
                            line: self.line,
                        }),
                        '"' => self.read_string(),
                        '-' | '0'..='9' => self.read_number(), // numbers may be -ve
                        't' | 'f' | 'n' => self.read_literal(),
                        '\r' | '\t' | ' ' | '\n' => self.skip_whitespace(),
                        _ => {
                            self.dump();
                            panic!("unknown char={xx} found at {}", self.line);
                        }
                    }
                }
            }
        }

        fn skip_whitespace(&mut self) {
            while self.not_at_end() {
                match self.peek() {
                    Some('\r' | '\t' | ' ') => {
                        self.advance();
                    }
                    Some('\n') => {
                        self.line += 1;
                        self.advance();
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        pub fn dump(&self) {
            println!("Dumping. {} items", self.tokens.len());
            for token in &self.tokens {
                println!("TOKEN => kind: {:?}, {}", token.kind, token.line);
            }
        }

        pub fn lex(&mut self) -> Vec<Token> {
            while self.not_at_end() {
                self.skip_whitespace();
                self.start = self.current;
                self.start();
            }

            self.tokens.push(Token {
                kind: Kind::EOF,
                line: self.line,
            });

            // return value
            return self.tokens.clone();
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

        // TODO: handle escape sequences.
        fn read_string(&mut self) {
            let mut s: String = String::new();

            while let Some(pat) = self.peek() {
                self.advance();
                if pat == '"' {
                    self.tokens.push(Token {
                        kind: Kind::String(s),
                        line: self.line,
                        // lexeme: (&self.content[self.start..self.current]).to_string(),
                    });
                    break;
                } else {
                    let st = self.resolve_string(pat);
                    s.push_str(&st);
                }
            }
        }

        fn resolve_string(&mut self, ch: char) -> String {
            if ch == '\\' {
                if let Some(pat) = self.peek() {
                    self.advance();
                    match pat {
                        'b' => return r"\b".to_string(),
                        't' => return r"\t".to_string(),
                        'n' => return r"\n".to_string(),
                        'f' => return r"\f".to_string(),
                        'r' => return r"\r".to_string(),
                        'u' | 'U' => {
                            // consume next hex didigt
                            let hex = &(self.content[self.current..self.current + 4]);
                            self.current += 4; // update idx;
                            let code_pt = u32::from_str_radix(hex, 16)
                                .expect("unable to convert hex codes {:?}");
                            if let Some(cc) = char::from_u32(code_pt) {
                                return cc.to_string();
                            } else {
                                panic!("Error converting code_pt {}", code_pt);
                            }
                        }
                        _ => return pat.to_string(),
                    }
                }
            }
            ch.to_string()
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
                } else if pat.is_ascii_whitespace() {
                    self.skip_whitespace();
                } else {
                    break;
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
                        break;
                    }
                }
            }

            // exponent part
            if exp {
                if let Some('-' | '+') = self.peek() {
                    self.advance();
                }
                while let Some(pat) = self.peek() {
                    if pat.is_ascii_whitespace() {
                        self.skip_whitespace();
                        break;
                    }
                    if pat.is_digit(10) {
                        number.push(pat);
                        self.advance();
                    } else {
                        self.dump();
                        panic!("unknown char {pat} found at {}", self.line);
                    }
                }
            }

            if number.ends_with("+") || number.ends_with("-") {
                self.dump();
                panic!("unknown sequence {number} found at {}", self.line);
            }

            let value = match number.parse::<f64>() {
                Ok(f) => f,
                Err(err) => {
                    self.dump();
                    panic!(
                        "unable to parse {} @ ln{} with error {}",
                        number, self.line, err
                    );
                }
            };

            self.tokens.push(Token {
                kind: Kind::Number(value),
                line: self.line,
            });
        }

        fn read_literal(&mut self) {
            while let Some('a'..='z') = self.peek() {
                self.advance();
            }

            let s = &self.content[self.start..self.current];
            let kd = match s {
                "true" => Kind::Boolean(true),
                "false" => Kind::Boolean(false),
                "null" => Kind::Null,
                _ => {
                    self.dump();
                    panic!("unknown literal `{s}` @ line{}", self.line);
                }
            };

            self.tokens.push(Token {
                kind: kd,
                line: self.line,
            });
        }
    }
}
