use super::lexer::{
    Token, 
    TokenType,
    tokenize,
};


/**
 * Test tokenizing key symbols "->".
 */
#[test]
fn test_tokenize_fn_return_type() {
    let test1_input: String = "->".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::FnType,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "()".
 */
#[test]
fn test_tokenize_none_type() {
    let test1_input: String = "()".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Type,    // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "!=".
 */
#[test]
fn test_tokenize_not_equal() {
    let test1_input: String = "!=".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "==".
 */
#[test]
fn test_tokenize_equal() {
    let test1_input: String = "==".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "+=".
 */
#[test]
fn test_tokenize_plus_equal() {
    let test1_input: String = "+=".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "-=".
 */
#[test]
fn test_tokenize_minus_equal() {
    let test1_input: String = "-=".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols ">=".
 */
#[test]
fn test_tokenize_grater_equal() {
    let test1_input: String = ">=".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "<=".
 */
#[test]
fn test_tokenize_less_equal() {
    let test1_input: String = "<=".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "&&".
 */
#[test]
fn test_tokenize_and() {
    let test1_input: String = "&&".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbols "||".
 */
#[test]
fn test_tokenize_or() {
    let test1_input: String = "||".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            3,                  // end_col
        )
    );
}
