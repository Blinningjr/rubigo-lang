mod span;
mod lexer;
mod parser;

use std::fs;
use lexer::{
    Token,
    TokenHandler,
};
use parser::{
    create_ast,
    Expression,
};


fn main() {
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);

    let test: String = "let test: i32 = 123;".to_string();
    println!("\nWith text:\n{}\n", test);

    let parsed: Expression = create_ast(&mut TokenHandler::new(test));
    println!("Parsed:\n{:#?}\n", parsed);

   // let mut tokens: Vec<Token> = Vec::new();
   // let mut token_handler: TokenHandler = TokenHandler::new(contents); 
   // let mut hungry: bool = true;
   // while hungry {
   //     match token_handler.next_token() {
   //         Ok(token) => tokens.push(token),
   //         Err(_err) => hungry = false,
   //     };
   // }
   // println!("Tokens:\n{:#?}", tokens);

}

