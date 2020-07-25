use super::tokenize_string;

use super::lexer::{
    Token, 
    TokenType,
    Lexer,
};


/**
 * Test tokenizing alphabetic identifiers.
 */
#[test]
fn test_tokenize_alphabetic_identifier() {
    let test1_input: String = "foo".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Identifier,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing alphanumeric identifiers.
 */
#[test]
fn test_tokenize_alphanumeric_identifier() {
    let test1_input: String = "foo1231".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Identifier,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing identifiers with underscore.
 */
#[test]
fn test_tokenize_underscore_identifier() {
    let test1_input: String = "foo_bar".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Identifier,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing string.
 */
#[test]
fn test_tokenize_string() {
    let test1_input: String = "\" ab c \"".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::String,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}



/**
 * Test tokenizing char.
 */
#[test]
fn test_tokenize_char() {
    let test1_input: String = "'a'".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Char,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


