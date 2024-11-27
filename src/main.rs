use std::fs::File;
use std::io::Read;

use json_parser::JsonParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path: String = args[1].to_string();

    let mut file: File = File::open(file_path).unwrap();
    
    let mut content: String = String::new();
    file.read_to_string(&mut content);

    let mut parser = JsonParser::new(&content);
    match parser.parse() {
        Ok(_) => println!("valid"),
        Err(_) => println!("invalid")
    }
}