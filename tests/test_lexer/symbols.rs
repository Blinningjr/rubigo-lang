use super::tokenize_string;

use super::lexer::{
    Token, 
    TokenType,
};


/**
 * Test tokenizing key symbols "->".
 */
#[test]
fn test_tokenize_fn_return_type() {
    let test1_input: String = "->".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::FnType,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "!=".
 */
#[test]
fn test_tokenize_not_equal() {
    let test1_input: String = "!=".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::NotEqual,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "==".
 */
#[test]
fn test_tokenize_equal() {
    let test1_input: String = "==".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Equal,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "+=".
 */
#[test]
fn test_tokenize_plus_equal() {
    let test1_input: String = "+=".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::PlusEquals,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "-=".
 */
#[test]
fn test_tokenize_minus_equal() {
    let test1_input: String = "-=".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::MinusEquals,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols ">=".
 */
#[test]
fn test_tokenize_grater_equal() {
    let test1_input: String = ">=".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::GreaterEqual,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "<=".
 */
#[test]
fn test_tokenize_less_equal() {
    let test1_input: String = "<=".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::LessEqual,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "&&".
 */
#[test]
fn test_tokenize_and() {
    let test1_input: String = "&&".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::And,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbols "||".
 */
#[test]
fn test_tokenize_or() {
    let test1_input: String = "||".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Or,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}

