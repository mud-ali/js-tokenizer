use std::{fs, io::Read};
use tokenizer;

fn main() {
    let mut file = fs::File::open("data/main.js").expect("File not found");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("failed to read file");
    println!("{}",code);
    println!("from mod: ");
    let token_delims = ["\n", " ", "\t", ";", ","];
    let ast = tokenizer::tokenizer::split_tokens(code, &token_delims);
    println!("{:#?}", ast);
}
