#![allow(dead_code)]
use core::panic;

use crate::HashMap;
use crate::Kind;
use crate::Token;
use crate::lexen::Lexer;

#[derive(Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    Strings(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl Json {
     pub fn isString(&self) -> bool {
         return matches!(self, Json::Strings(_))
     }
 
     pub fn isBoolean(&self) -> bool {
         return matches!(self, Json::Boolean(_))
     }
 
     pub fn isNumber(&self) -> bool {
         return matches!(self, Json::Boolean(_))
     }

     pub fn isArray(&self) -> bool {
         return matches!(self, Json::Array(_))
     }
 
     pub fn isObject(&self) -> bool {
         return matches!(self, Json::Object(_))
     }
 
     pub fn isNullOrEmptyNode(&self) -> bool {
         return matches!(self, Json::Null)
     }
 
     pub fn asText(&self) -> String {
         if matches!(self, Json::Boolean(_) | Json::Strings(_) | Json::Number(_)) {
             match self {
                 Json::Boolean(b) => b.to_string(),
                 Json::Number(num) => num.to_string(),  
                 Json::Strings(st) => st.clone(),  
                 Json::Null => "null".to_string(),  
                 _ => panic!("only primitive types can be converted to text")
             }
         } else {
             "".to_string()
         }
     }
 
    pub fn path(&self, name: &str) -> &Json {
            match self {
            Json::Object(map) => {
                if let Some(val ) = map.get(name) {
                    val
                } else {
                    &Json::Null
                }
            }
            _ => return &Json::Null,
        }
     }
 
     pub fn atIndex(&self, index: usize) -> &Json {
        match self {
            Json::Array(vec) => {
                if let Some(json) = vec.get(index) {
                    json
                } else {
                    &Json::Null
                }
            }
            _ => return &Json::Null,
        }
     }
 }



/// Parsers job is to consume tokens, make sure that they adhere to the
/// language's grammar and produces a Json result.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
}

impl Parser {
    pub fn new(_tokens: Vec<Token>) -> Self {
        Self {
            tokens: _tokens,
            start: 0,
            current: 0,
        }
    }

    pub fn from_string(s: String) -> Self {
        let mut lexer = Lexer::new(s);
        return Parser::new(lexer.lex());
    }

    // member = string name-separator value
    pub fn parse(&mut self) -> crate::parsen::Json {
        match self.peek() {
            Some(x) => match x.get_kind() {
                Kind::BeginObject => self.parse_object(),
                Kind::BeginArray => self.parse_array(),
                Kind::String(_) | Kind::Boolean(_) | Kind::Null | Kind::Number(_) => {
                    let json = self.resolve_type();
                    self.advance();
                    return json;
                }
                _ => panic!(),
            },
            None => panic!(),
        }
    }

    // object = begin-object [ member *( value-separator member ) ]
    //  end-object
    fn parse_object(&mut self) -> crate::parsen::Json {
        let mut heap_map: Box<HashMap<String, Json>> = Box::new(HashMap::<String, Json>::new());
        // consume expected {
        self.consume(Kind::BeginObject);

        while self.not_exhausted() {
            match self.peek() {
                Some(ref token) => {
                    if token.get_kind().clone() == Kind::EndObject {
                        break;
                    } else {
                        // we must find a string here.
                        match token.get_kind() {
                            Kind::String(s) => {
                                let s = s.clone();
                                self.advance();
                                self.consume(Kind::NameSeparator); 
                                
                                let json: Json = self.parse();
                                heap_map.insert(s, json);
                            }
                            _ => panic!("expected string as key found {:?} instead", token.get_kind()),
                        }
                    }
                },
                None => panic!("Exhausted values parsing object."),
            }

            if !self.expect_to_find(Kind::ValueSeparator) {
                break;
            } else {
                self.advance(); // consume  valueSep(,)
            }
        }

        self.consume(Kind::EndObject);
        Json::Object(heap_map)
    }

    // array = begin-array [ value *( value-separator value ) ] end-array
    // value = false / null / true / object / array / number / string
    fn parse_array(&mut self) -> crate::parsen::Json {
        let mut v = Vec::<crate::parsen::Json>::new();
        self.consume(Kind::BeginArray);

        while self.not_exhausted() {
            match self.peek() {
                Some(ref token) => {
                    if token.get_kind().clone() == Kind::EndArray {
                        break;
                    } else {
                        let json: Json = self.parse();
                        v.push(json);
                    };
                }
                None => panic!("Exhausted values parsing array."),
            }

            // if value separator not found, stop parsing array
            if !self.expect_to_find(Kind::ValueSeparator) {
                break;
            } else {
                self.advance(); 
            }
        }
        self.consume(Kind::EndArray);
        Json::Array(v)
    }

    fn resolve_type(&self) -> crate::parsen::Json {
        match self.peek() {
            Some(token) => {
                match token.get_kind() {
                    Kind::Boolean(b) => Json::Boolean(b),
                    Kind::Null => Json::Null,            // null
                    Kind::String(s) => Json::Strings(s), // string
                    Kind::Number(f) => Json::Number(f),  // number
                    _ => panic!("matched unexpected {:?}", token),
                }
            }
            None => panic!(""),
        }
    }

    fn consume(&mut self, check: Kind) {
        let b: bool = self.expect_to_find(check);
        if b {
            self.advance();
        } else {
            panic!("unexpected token {:?}", self.peek());
        }
    }

    fn expect_to_find(&self, check: Kind) -> bool {
        if let Some(ref kd) = self.peek() {
            if kd.get_kind() == check { true } else { false }
        } else {
            // exhausted input
            return false;
        }
    }

    fn not_exhausted(&self) -> bool {
        return self
            .tokens
            .get(self.current)
            .expect("Illegal state. Tokens exhausted;")
            .get_kind()
            != Kind::EOF;
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        let ch: Option<&Token> = self.tokens.get(self.current);
        self.current += 1;
        ch
    }
}

