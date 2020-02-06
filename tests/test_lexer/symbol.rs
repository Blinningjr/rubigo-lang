use super::lexer::{
    Token, 
    TokenType,
    tokenize,
};


/**
 * Test tokenizing key symbol ' '.
 */
#[test]
fn test_tokenize_blank_space() {
    let test1_input: String = " ".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens.len(), 0);
}


/**
 * Test tokenizing key symbol '\n'.
 */
#[test]
fn test_tokenize_new_line() {
    let test1_input: String = "\n".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens.len(), 0);
}


/**
 * Test tokenizing key symbol ':'.
 */
#[test]
fn test_tokenize_type_dec() {
    let test1_input: String = ":".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::TypeDec, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '"'.
 */
#[test]
fn test_tokenize_string() {
    let test1_input: String = '"'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::String,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '{'.
 */
#[test]
fn test_tokenize_body_start() {
    let test1_input: String = '{'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::BodyStart,   // token_type
            test1_input,            // value
            1,                      // line
            1,                      // start_col
            2,                      // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '}'.
 */
#[test]
fn test_tokenize_body_end() {
    let test1_input: String = '}'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::BodyEnd, // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '('.
 */
#[test]
fn test_tokenize_parenthesis_start() {
    let test1_input: String = '('.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::ParenthesisStart,    // token_type
            test1_input,                    // value
            1,                              // line
            1,                              // start_col
            2,                              // end_col
        )
    );
}


/**
 * Test tokenizing key symbol ')'.
 */
#[test]
fn test_tokenize_parenthesis_end() {
    let test1_input: String = ')'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::ParenthesisEnd,  // token_type
            test1_input,                // value
            1,                          // line
            1,                          // start_col
            2,                          // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '&'.
 */
#[test]
fn test_tokenize_borrow() {
    let test1_input: String = '&'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Borrow,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '='.
 */
#[test]
fn test_tokenize_equals() {
    let test1_input: String = '='.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '!'.
 */
#[test]
fn test_tokenize_not() {
    let test1_input: String = '!'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '+'.
 */
#[test]
fn test_tokenize_plus() {
    let test1_input: String = '+'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '-'.
 */
#[test]
fn test_tokenize_minus() {
    let test1_input: String = '-'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '/'.
 */
#[test]
fn test_tokenize_divide() {
    let test1_input: String = '/'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '*'.
 */
#[test]
fn test_tokenize_multiply() {
    let test1_input: String = '*'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '%'.
 */
#[test]
fn test_tokenize_modulus() {
    let test1_input: String = '%'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '<'.
 */
#[test]
fn test_tokenize_less_then() {
    let test1_input: String = '<'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol '>'.
 */
#[test]
fn test_tokenize_greater_then() {
    let test1_input: String = '>'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Op,      // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );
}


/**
 * Test tokenizing key symbol ';'.
 */
#[test]
fn test_tokenize_end_expression() {
    let test1_input: String = ';'.to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::EndExpression,   // token_type
            test1_input,                // value
            1,                          // line
            1,                          // start_col
            2,                          // end_col
        )
    );
}
