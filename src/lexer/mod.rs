pub mod token;

pub use crate::lexer::token::{
    Token,
    TokenHandler,
};

pub fn generateTokens(input: String) -> Vec<Token> {
    let token_handler = TokenHandler::new(input); 
    return  token_handler.get_tokens();
}