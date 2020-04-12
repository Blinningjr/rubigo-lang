#![allow(dead_code)]

pub mod types;
pub mod expressions;
pub mod operations;
pub mod ast;

pub use types::{
    Type,
    Literal,
};

pub use expressions::{
    Expression,
};

use super::lexer::token::{
    Token,
    TokenHandler,
    // TokenType,
};


/**
 * Parses the tokens and returns a Ast. 
 */
pub fn parse_tokens(tokens_handler: &mut TokenHandler) {
    while tokens.len() > 0 {
        // let token: Token = tokens.remove(0);
        check_token(tokens);
    }
}


/**
 * Checks the first token in tokens and calls its parser.
 */
fn check_token(tokens: &mut Vec<Token>) {
    let token: Token = tokens[0].clone();
    match token.get_type() {
        // TokenType::Fn => parse_fn(tokens),
        // TokenType::Number => ,
        _ => panic!("Syntax Error: Expected token ..."),
    };
}


/**
 * Parses a function token and all its related tokens. 
 */
// fn parse_fn(tokens: &mut Vec<Token>) {
    
// }


/**
 * Collects all parameters into a list.
*/
fn collect_parameters() {

}
