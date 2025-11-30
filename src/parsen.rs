#![allow(dead_code)]
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

/// Parsers job is to consume tokens, make sure that they adhere to the
/// language's grammar and produces a Json result.
#[derive(Debug)]
struct Parser {
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

    // pub fn parse(&mut self) -> crate::parse::Json {
    //
    // }
    //
    // object = begin-object [ member *( value-separator member ) ]
    //  end-object
    pub fn parse(&mut self) -> crate::parsen::Json {
        match self.peek() {
            Some(x) => match x.get_kind() {
                Kind::BeginObject => self.parse_object(),
                Kind::BeginArray => self.parse_array(),
                Kind::String(_) | Kind::Boolean(_) | Kind::Null | Kind::Number(_) => {
                    self.resolve_type()
                }
                _ => panic!(),
            },
            None => panic!(),
        }
    }

    pub fn parse_object(&mut self) -> crate::parsen::Json {
        let mut heap_map = Box::new(HashMap::<String, Json>::new());
        if self.expect_to_find(Kind::BeginObject) {
            self.advance();
        } else {
            panic!("unexpected token {:?}", self.peek());
        } // matched {
        //Account { name, language, .. } => {
        // ui.greet(&name, &language);
        // ui.show_settings(&account);  // error: borrow of moved value: `account`

        while self.not_exhausted() {
            // TODO: extract string from the Token
            let s = String::from("ladkfla");
            self.advance(); // conusme name
            self.consume(Kind::NameSeparator); // consume separator 
            let json = self.parse();
            heap_map.insert(s, json);
        }
        self.consume(Kind::EndObject);
        Json::Object(heap_map)
    }

    pub fn parse_array(&mut self) -> crate::parsen::Json {
        let mut v = Vec::<crate::parsen::Json>::new();
        if self.expect_to_find(Kind::BeginArray) {
            self.advance();
        } else {
            panic!("unexpected token {:?}", self.peek());
        } // matched [

        while self.not_exhausted() {
            match self.peek() {
                // match an object
                Some(ref token) => {
                    if token.get_kind().clone() != Kind::EndArray
                        || token.get_kind().clone() != Kind::ValueSeparator
                    {
                        v.push(self.resolve_type());
                        self.advance();
                    };
                }
                None => panic!("Exhausted values parsing array."),
            }

            if !self.expect_to_find(Kind::ValueSeparator) {
                break;
            } else {
                self.advance(); // consume  valueSep(,)
            }
        }

        if self.expect_to_find(Kind::EndArray) {
            self.advance();
        } else {
            panic!("unexpected token {:?}", self.peek());
        }
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
        if self.expect_to_find(check) {
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

    // member = string name-separator value
    // value = false / null / true / object / array / number / string
    // array = begin-array [ value *( value-separator value ) ] end-array
}
