mod lexer;
mod parser;
pub mod error_handler;

use std::fs;
use lexer::{
    Token,
    TokenHandler,
    fsm_start,
};


fn main() {
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

//    let mut tokens: Vec<Token> = lexer::tokenize(contents);
//    println!("Tokens:\n{:#?}", tokens);

    let mut token_handler: TokenHandler = TokenHandler::new(contents); 
    while token_handler.hungry() {
        let (ch, look_a_head): (Option<char>, Option<char>) = token_handler.next_char();
        fsm_start(&mut token_handler, ch.unwrap(), look_a_head);
    }
    println!("Tokens:\n{:#?}", token_handler.get_tokens());

//    parser::parse_tokens(&mut tokens);
}

