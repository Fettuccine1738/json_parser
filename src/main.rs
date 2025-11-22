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
    let lexer: Lexer = Lexer::new(read);
}
