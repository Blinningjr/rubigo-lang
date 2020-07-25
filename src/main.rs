mod span;
mod lexer;
mod parser;

use std::fs;
use lexer::{
    Token,
    Lexer,
};

use parser::{
    Parser,
};


fn main() {
    let filename = "example-code/example.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("Input text:\n{}", contents);

    println!("Ast: \n{:#?}\n", Parser::parse(contents));    
}

