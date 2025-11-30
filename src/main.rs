use json_parser::Token;
use json_parser::lexen;
use json_parser::lexen::Lexer;
use json_parser::read_file;

// contains the array and object samples from the RFC8259
//https://www.rfc-editor.org/rfc/rfc8259.txt
const SAMPLE_OBJECT_FILE: &str = "json_obj.txt";
const SAMPLE_ARRAY_FILE: &str = "array.txt";

fn main() {
    let read = match read_file(SAMPLE_ARRAY_FILE) {
        Ok(suc) => suc,
        Err(err) =>
        // eprintln!("file read failure, {}", err);
        {
            "error".to_string()
        }
    };

    println!("{}", read);
    let mut lexer: Lexer = Lexer::new(read);
    let tokens: Vec<Token> = lexer.lex();
    // lexer.dump();
    for t in &tokens {
        println!("{:?}", t);
    }
}
