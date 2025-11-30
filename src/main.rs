use json_parser::Token;
use json_parser::lexen;
use json_parser::lexen::Lexer;
use json_parser::read_file;

fn main() {
    let read = match read_file("json_obj.txt") {
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
