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

use super::lexer::{
    Token,
    TokenHandler,
    TokenType,
};


/**
 * Parses the tokens and returns a Ast. 
 */
pub fn create_ast(token_handler: &mut TokenHandler) -> Expression {
    return check_token(token_handler);
   // while tokens.len() > 0 {
   //     // let token: Token = tokens.remove(0);
   //     check_token(tokens);
   // }
}


/**
 * Checks the first token in tokens and calls its parser.
 */
fn check_token(token_handler: &mut TokenHandler) -> Expression {
    let token: Token = token_handler.next_token().unwrap();
    return match token.get_type() {
        TokenType::Char => Expression::Math(parse_char(token_handler, & token)),
        TokenType::Number => Expression::Math(parse_number(& token)),
        TokenType::FloatNumber => Expression::Math(parse_float_number(& token)),
        _ => panic!("Syntax error: Token not implemented {:?}", token),
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


/**
 * Parses token of type Char into literal char.
 */
fn parse_char(token_handler: &mut TokenHandler, token: & Token) -> Span<Literal> {
    match token.get_type() {
        TokenType::Char => {
            let mut tokens: Vec<Token> = parse_until(token_handler, TokenType::Char);
            println!("{:?}", tokens);
            match tokens.pop().unwrap().get_type() {
                TokenType::Char => {
                    if tokens.len() == 1 {
                        let res_token: Token = tokens.pop().unwrap(); 
                        return match res_token.get_value().parse::<char>() {
                            Ok(val) => Span::new(Literal::Char(val), res_token.get_line(), res_token.get_offset()),
                            Err(_e) => panic!("Syntax error: could not parse f32"),
                        }; 
                    } else {
                        panic!("Syntax Error: can't parse char");
                    }
                }, 
                _ => panic!("Syntax Error: Missing char end '"),
            }; 
        },
        _ => panic!("Syntax error: could not parse Char"),
    };
}


/**
 * Takes a vec of tokens.
 * Returns the concatinated String that the tokens make.
 */
fn get_tokens_string(tokens: & Vec<Token>) -> String {
    let mut result: String = "".to_string();
    for token in tokens.iter() {
       result.push_str(& token.get_value()); 
    }
    return result;
}


/**
 * Returns all tokens before and including the first token of type token_type it finds.
 */
fn parse_until(token_handler: &mut TokenHandler, token_type: TokenType) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while token_handler.hungry() {
        let token: Token = token_handler.next_token().unwrap();
        tokens.push(token.clone());
        if token.get_type() == token_type {
            return tokens;
        }
    }
    return tokens;
}


/**
 * Parses token of type Number into literal i32.
 */
fn parse_number(token: & Token) -> Span<Literal> {
    match token.get_type() {
        TokenType::Number => {
            return match token.get_value().parse::<i32>() {
                Ok(val) => Span::new(Literal::I32(val), token.get_line(), token.get_offset()),
                Err(_e) => panic!("Syntax error: could not parse i32"),
            }; 
        },
        _ => panic!("Syntax error: could not parse i32"),
    };
}


/**
 * Parses token of type FloatNumber into literal f32.
 */
fn parse_float_number(token: & Token) -> Span<Literal> {
    match token.get_type() {
        TokenType::FloatNumber => {
            return match token.get_value().parse::<f32>() {
                Ok(val) => Span::new(Literal::F32(val), token.get_line(), token.get_offset()),
                Err(_e) => panic!("Syntax error: could not parse f32"),
            }; 
        },
        _ => panic!("Syntax error: could not parse f32"),
    };
}

