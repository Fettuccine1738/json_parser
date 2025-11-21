use json_parser::lexen;

fn main() {
    println!("Hello, world!");
    let s = "number-".to_string();
    let b: bool = s.ends_with("-");
    println!("s ends with {b}");
}
