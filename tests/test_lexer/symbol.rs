use super::tokenize_string;

use super::lexer::{
    Token, 
    TokenType,
};


/**
 * Test tokenizing key symbol ':'.
 */
#[test]
fn test_tokenize_type_dec() {
    let test1_input: String = ":".to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::TypeDec, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '"'.
 */
#[test]
fn test_tokenize_string() {
    let test1_input: String = '"'.to_string();
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
 * Test tokenizing key symbol '{'.
 */
#[test]
fn test_tokenize_body_start() {
    let test1_input: String = '{'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::BodyStart,   // token_type
            test1_input,            // value
            1,                      // line
            1,                      // offset
        )
    );
}


/**
 * Test tokenizing key symbol '}'.
 */
#[test]
fn test_tokenize_body_end() {
    let test1_input: String = '}'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::BodyEnd, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '('.
 */
#[test]
fn test_tokenize_parenthesis_start() {
    let test1_input: String = '('.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::ParenthesisStart,    // token_type
            test1_input,                    // value
            1,                              // line
            1,                              // offset
        )
    );
}


/**
 * Test tokenizing key symbol ')'.
 */
#[test]
fn test_tokenize_parenthesis_end() {
    let test1_input: String = ')'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::ParenthesisEnd,  // token_type
            test1_input,                // value
            1,                          // line
            1,                          // offset
        )
    );
}


/**
 * Test tokenizing key symbol '&'.
 */
#[test]
fn test_tokenize_borrow() {
    let test1_input: String = '&'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Borrow,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '='.
 */
#[test]
fn test_tokenize_equals() {
    let test1_input: String = '='.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Equals,      // token_type
            test1_input,            // value
            1,                      // line
            1,                      // offset
        )
    );
}


/**
 * Test tokenizing key symbol '!'.
 */
#[test]
fn test_tokenize_not() {
    let test1_input: String = '!'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Not,     // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '+'.
 */
#[test]
fn test_tokenize_plus() {
    let test1_input: String = '+'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Plus,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '-'.
 */
#[test]
fn test_tokenize_minus() {
    let test1_input: String = '-'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Minus,   // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '/'.
 */
#[test]
fn test_tokenize_divide() {
    let test1_input: String = '/'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::ForwardSlash,    // token_type
            test1_input,                // value
            1,                          // line
            1,                          // offset
        )
    );
}


/**
 * Test tokenizing key symbol '*'.
 */
#[test]
fn test_tokenize_multiply() {
    let test1_input: String = '*'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Star,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // offset
        )
    );
}


/**
 * Test tokenizing key symbol '%'.
 */
#[test]
fn test_tokenize_modulus() {
    let test1_input: String = '%'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Modilus,     // token_type
            test1_input,            // value
            1,                      // line
            1,                      // offset
        )
    );
}


/**
 * Test tokenizing key symbol '<'.
 */
#[test]
fn test_tokenize_less_then() {
    let test1_input: String = '<'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::LessThen,    // token_type
            test1_input,            // value
            1,                      // line
            1,                      // offset
        )
    );
}


/**
 * Test tokenizing key symbol '>'.
 */
#[test]
fn test_tokenize_greater_then() {
    let test1_input: String = '>'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::GreaterThen,      // token_type
            test1_input,                // value
            1,                          // line
            1,                          // offset
        )
    );
}


/**
 * Test tokenizing key symbol ';'.
 */
#[test]
fn test_tokenize_end_expression() {
    let test1_input: String = ';'.to_string();
    let test1_tokens: Vec<Token> = tokenize_string(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::SemiColon,       // token_type
            test1_input,                // value
            1,                          // line
            1,                          // offset
        )
    );
}

