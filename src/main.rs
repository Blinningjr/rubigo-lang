pub mod lexer;
mod parser;


use std::fs;
use lexer::Token;


fn main() {
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    let mut tokens: Vec<Token> = lexer::tokenize(contents);
    println!("Tokens:\n{:#?}", tokens);

    parser::parse_tokens(&mut tokens);
}
