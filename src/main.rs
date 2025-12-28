#![allow(dead_code)]
use json_parser::Token;
use json_parser::lexen::Lexer;
use json_parser::parsen::Parser;
use json_parser::read_file;

/// contains the array and object samples from the RFC8259
/// https://www.rfc-editor.org/rfc/rfc8259.txt
const SAMPLE_OBJECT_FILE: &str = "json_obj.txt";
const SAMPLE_ARRAY_FILE: &str = "array.txt";

fn main() {
    let text = "Hello \\u03BB world";
    println!("{text}");
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
    println!("json.isArray = {}", json.is_array());
    println!("json.isObject = {}", json.is_object());
    println!("json.isNull = {}", json.is_null_or_empty());
    println!("json.isNumber = {}", json.is_number());
    println!("json.isString = {}", json.is_string());
    println!("json.isBoolean = {}", json.is_boolean());

    let s: String = json.at_index(0).path("precision").as_text();
    // let lambda = "\\u03BB";
    // assert_eq!(s, rhs);
    //
    let latitude: String = json.at_index(0).path("Latitude").as_text();
    let longitude: String = json.at_index(0).path("Longitude").as_text();
    let address: String = json.at_index(0).path("Address").as_text();
    let city: String = json.at_index(0).path("City").as_text();
    let state: String = json.at_index(0).path("State").as_text();
    let zip: String = json.at_index(0).path("Zip").as_text();
    let country: String = json.at_index(0).path("Country").as_text();

    println!("precision = {}", s);
    println!("latitude = {}", latitude);
    println!("longitude = {}", longitude);
    println!("address = {}", address);
    println!("city = {}", city);
    println!("state = {}", state);
    println!("zip = {}", zip);
    println!("country = {}", country);
}
