#![allow(dead_code)]

#[path = "../../src/lexer/mod.rs"]
pub mod lexer;

mod idents;
mod numbers;
mod reserved;
mod symbol;
mod symbols;

use lexer::{
    Token,
    TokenHandler,
};


/**
 * converts the whole string into a vec of tokens.
 */
pub fn tokenize_string(input: String) -> Vec<Token> {
    let mut hungry: bool = true;
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_handler: TokenHandler = TokenHandler::new(input);
    while hungry {
        match token_handler.next_token() {
            Ok(token) => tokens.push(token),
            Err(_err) => hungry = false,
        };
    }
    return tokens;
}

