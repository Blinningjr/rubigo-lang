use super::tokenize_string;

use super::lexer::{
    Token, 
    TokenType,
};


/**
 * Test tokenizing key identifier fn.
 */
#[test]
fn test_tokenize_fn() {
    let test1_input: String = "fn".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Fn,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier while.
 */
#[test]
fn test_tokenize_while() {
    let test1_input: String = "while".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::While,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier if.
 */
#[test]
fn test_tokenize_if() {
    let test1_input: String = "if".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::If,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier else.
 */
#[test]
fn test_tokenize_else() {
    let test1_input: String = "else".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Else,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier let.
 */
#[test]
fn test_tokenize_let() {
    let test1_input: String = "let".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Let,     // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier mut.
 */
#[test]
fn test_tokenize_mut() {
    let test1_input: String = "mut".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Mut,     // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier false.
 */
#[test]
fn test_tokenize_false() {
    let test1_input: String = "false".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Boolean, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier true.
 */
#[test]
fn test_tokenize_true() {
    let test1_input: String = "true".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Boolean, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier bool.
 */
#[test]
fn test_tokenize_bool() {
    let test1_input: String = "bool".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::TBool,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier i32.
 */
#[test]
fn test_tokenize_i32() {
    let test1_input: String = "i32".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Ti32,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier f32.
 */
#[test]
fn test_tokenize_f32() {
    let test1_input: String = "f32".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Tf32,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier String.
 */
#[test]
fn test_tokenize_string() {
    let test1_input: String = "String".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::TString,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key identifier return.
 */
#[test]
fn test_tokenize_return() {
    let test1_input: String = "return".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Return,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}

