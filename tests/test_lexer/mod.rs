#![allow(dead_code)]

#[path = "../../src/span.rs"]
mod span;

#[path = "../../src/lexer/mod.rs"]
pub mod lexer;

mod identifiers;
mod numbers;
mod reserved;
mod symbol;
mod symbols;


use lexer::{
    Token,
    Lexer,
};


/**
 * converts the whole string into a vec of tokens.
 */
pub fn tokenize_string(input: String) -> Vec<Token> {
    let mut hungry: bool = true;
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer: Lexer = Lexer::new(input);
    while hungry {
        match lexer.next_token(false) {
            Ok(token) => tokens.push(token),
            Err(_err) => hungry = false,
        };
    }
    return tokens;
}

