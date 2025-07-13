use std::{fs, io::Read};

fn main() {
    let mut file = fs::File::open("data/main.js").expect("File not found");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("failed to read file");
    println!("{}",code);
}
