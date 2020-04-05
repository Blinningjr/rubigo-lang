use super::tokenize_string;

use super::lexer::{
    Token, 
    TokenType,
    TokenHandler,
};


/**
 * Test tokenizing alphabetic identifiers.
 */
#[test]
fn test_tokenize_alphabetic_ident() {
    let test1_input: String = "foo".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Ident,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            4,                  // end_col
        )
    );
}


/**
 * Test tokenizing alphanumeric identifiers.
 */
#[test]
fn test_tokenize_alphanumeric_ident() {
    let test1_input: String = "foo1231".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Ident,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            8,                  // end_col
        )
    );
}


/**
 * Test tokenizing identifiers with underscore.
 */
#[test]
fn test_tokenize_underscore_ident() {
    let test1_input: String = "foo_bar".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Ident,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            8,                  // end_col
        )
    );
}

