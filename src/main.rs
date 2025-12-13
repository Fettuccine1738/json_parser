#![allow(dead_code)]
use json_parser::Token;
use json_parser::parsen::Parser;
use json_parser::lexen::Lexer;
use json_parser::read_file;

/// contains the array and object samples from the RFC8259
/// https://www.rfc-editor.org/rfc/rfc8259.txt
const SAMPLE_OBJECT_FILE: &str = "json_obj.txt";
const SAMPLE_ARRAY_FILE: &str = "array.txt";

fn main() {
    let read = match read_file(SAMPLE_ARRAY_FILE) {
        Ok(suc) => suc,
        Err(err) => {
            eprintln!("file read failure, {}", err);
            return;
        }
    };

    println!("{}", read);
    let mut lexer: Lexer = Lexer::new(read);
    let tokens: Vec<Token> = lexer.lex();
    for t in &tokens {
        println!("{:?}", t);
    }

    let mut parser = Parser::new(tokens);
    let json = parser.parse();
    println!("json.isArray = {}", json.isArray());
    println!("json.isObject = {}", json.isObject());
    println!("json.isNull = {}", json.isNullOrEmptyNode());
    println!("json.isNumber = {}", json.isNumber());
    println!("json.isString = {}", json.isString());
    println!("json.isBoolean = {}", json.isBoolean());

    let s: String = json.atIndex(0).path("precision").asText();
    assert_eq!(s, "zip");

    let latitude: String = json.atIndex(0).path("Latitude").asText();
    let longitude: String = json.atIndex(0).path("Longitude").asText();
    let address: String = json.atIndex(0).path("Address").asText();
    let city: String = json.atIndex(0).path("City").asText();
    let state: String = json.atIndex(0).path("State").asText();
    let zip: String = json.atIndex(0).path("Zip").asText();
    let country: String = json.atIndex(0).path("Country").asText();

    println!("latitude = {}", latitude);
    println!("longitude = {}", longitude);
    println!("address = {}", address);
    println!("city = {}", city);
    println!("state = {}", state);
    println!("zip = {}", zip);
    println!("country = {}", country);
}
