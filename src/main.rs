mod lexer;
//mod parser;

use std::fs;
use lexer::{
    Token,
    TokenHandler,
};


fn main() {
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

//    parse_tokens(&mut TokenHandler::new(contents))

    let mut tokens: Vec<Token> = Vec::new();
    let mut token_handler: TokenHandler = TokenHandler::new(contents); 
    let mut hungry: bool = true;
    while hungry {
        match token_handler.next_token() {
            Ok(token) => tokens.push(token),
            Err(_err) => hungry = false,
        };
    }
    println!("Tokens:\n{:#?}", tokens);

}

