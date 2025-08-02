use std::{fs, io::Read};
use tokenizer;

fn main() {
    let mut file = fs::File::open("data/main.js").expect("File not found");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("failed to read file");
    println!("[DEBUG] from tokenizer module: ");
    let token_delims = ["\n", " ", "\t", ";", ",", "(", ")"];
    let ast = tokenizer::tokenizer::split_tokens(code.trim().to_string(), &token_delims);

    println!("[DEBUG] end tokenizer output");

    let out_dir = "out";
    if !std::path::Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Unable to create output directory");
    }
    fs::write("out/tree", format!("{:#?}", ast)).expect("Unable to write file");

    println!("[DEBUG] from syntax-highlighter module:");
    let colored = tokenizer::syntax_highlighter::highlight(ast);
    println!("[DEBUG] end syntax-highlighter debug output. Printing highlighted code now:\n\n");

    print!(" ");
    for item in colored {
        print!("{} ", item);
    }
    println!();
}
