use std::fs;

mod lexer;
use crate::lexer::Token;

fn main() {
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let tokens: Vec<Token> = lexer::generateTokens(contents);

    println!("Tokens:\n{:?}", tokens);
}
