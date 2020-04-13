#![allow(dead_code)]

pub mod types;
pub mod expressions;
pub mod operations;
pub mod ast;
pub mod span;

pub use span::Span;
pub use ast::Ast;

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
pub fn parse_tokens(token_handler: &mut TokenHandler) -> Expression {
    return check_token(token_handler);
   // while tokens.len() > 0 {
   //     // let token: Token = tokens.remove(0);
   //     check_token(tokens);
   // }
}


/**
 * Checks the first token in tokens and calls its parser.
 */
fn check_token(token_handler: &mut TokenHandler) {
    let token: Token = token_handler.next_token();
    match token {
        TokenType::Number => ,
        _ => panic!("Syntax error: Token not implemented {}", token),
    }
    //let token: Token = tokens[0].clone();
    //match token.get_type() {
    //    // TokenType::Fn => parse_fn(tokens),
    //    // TokenType::Number => ,
    //    _ => panic!("Syntax Error: Expected token ..."),
    //};
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


fn parse_number(& token: Token) -> Literal {
    
}

