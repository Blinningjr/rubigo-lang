pub mod token_type;
pub mod token;
pub mod token_handler;
pub mod reserved;

pub use token_type::TokenType;
pub use token::Token;
pub use token_handler::TokenHandler;

use reserved::{
    check_reserved_idents,
    check_symbols,
    check_symbol,
};


/**
 * Generates vector of tokens from a String.
 */
//pub fn tokenize(input: String) -> Vec<Token> {
//    let mut token_handler: TokenHandler = TokenHandler::new(input); 
//    while token_handler.hungry() {
//        let (ch, look_a_head): (Option<char>, Option<char>) = token_handler.next_char();
//        fsm_start(&mut token_handler, ch.unwrap(), look_a_head);
//    }
//    return  token_handler.get_tokens();
//}


/**
 * Starts a finite state machine for consuming and classifying a token.
 */
pub fn fsm_start(token_handler: &mut TokenHandler, ch: char, look_a_head: Option<char>) {
    if ch.is_alphabetic() {
        token_handler.consume();
        // Make it call reserved FSM:s.
        fsm_ident(token_handler);
    } else if ch.is_numeric() {
        token_handler.consume();
        fsm_number(token_handler);
    } else {
        let mut is_tokenized: bool;
        
        match look_a_head {
            Some(look_a_head) => is_tokenized = check_symbols(token_handler, ch, look_a_head),
            None => is_tokenized = false,
        };
        
        if !is_tokenized {
            is_tokenized = check_symbol(token_handler, ch);
        }

        if !is_tokenized {
            token_handler.discard();
        }
    }
}


/**
 * FSM for converting string to Token of type ident.
 */
pub fn fsm_ident(token_handler: &mut TokenHandler) {
    if token_handler.hungry() {
        let (ch, _look_a_head): (Option<char>, Option<char>) = token_handler.next_char();
        // TODO: Fix so '_' is not hard coded.
        if ch.unwrap().is_alphanumeric() || ch.unwrap() == '_' {
            token_handler.consume();
            fsm_ident(token_handler);
        } else {
            fsm_ident_end(token_handler);
        }
    } else {
        fsm_ident_end(token_handler);
    } 
}


/**
 * Checks if the ident is reserved and stores the correct token.
 */
fn fsm_ident_end(token_handler: &mut TokenHandler) {
    let token_value: &str = token_handler.get_token_value();
    let token_type: TokenType = check_reserved_idents(token_value);
    token_handler.next_token(token_type);
}


/**
 * FSM for converting string to Token of type number. 
 */
fn fsm_number(token_handler: &mut TokenHandler) {
    if token_handler.hungry() {
        let tup: (Option<char>, Option<char>) = token_handler.next_char();
        let ch: char = tup.0.unwrap();

        if ch.is_numeric() {
            token_handler.consume();
            fsm_number(token_handler);
        } else {
            match ch {
                '.' => {
                    match tup.1 {
                        Some(look_a_head) => {
                            if look_a_head.is_numeric() {
                                token_handler.consume();
                                token_handler.consume();
                                fsm_number(token_handler);
                            } else {
                                token_handler.next_token(TokenType::Number);
                            }
                        },
                        None => token_handler.next_token(TokenType::Number),
                    };
                },
                _ => token_handler.next_token(TokenType::Number),
            };
        }
    } else {
        token_handler.next_token(TokenType::Number);
    }
}

