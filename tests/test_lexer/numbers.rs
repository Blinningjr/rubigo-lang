use super::lexer::{
    Token, 
    TokenType,
    tokenize,
};


/**
 * Test tokenizing int numbers.
 */
#[test]
fn test_tokenize_int_number() {
    let test1_input: String = "1".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Number,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            2,                  // end_col
        )
    );

    let test2_input: String = "34567832".to_string();
    let test2_tokens: Vec<Token> = tokenize(test2_input.clone());
    assert_eq!(test2_tokens[0], 
        Token::new(
            TokenType::Number,  // token_type
            test2_input,        // value
            1,                  // line
            1,                  // start_col
            9,                  // end_col
        )
    );
}


/**
 * Test tokenizing float numbers.
 */
#[test]
fn test_tokenize_float_number() {
    let test1_input: String = "3.4".to_string();
    let test1_tokens: Vec<Token> = tokenize(test1_input.clone());
    assert_eq!(test1_tokens[0], 
        Token::new(
            TokenType::Number,  // token_type
            test1_input,        // value
            1,                  // line
            1,                  // start_col
            4,                  // end_col
        )
    );

    let test2_input: String = "32131.4453".to_string();
    let test2_tokens: Vec<Token> = tokenize(test2_input.clone());
    assert_eq!(test2_tokens[0], 
        Token::new(
            TokenType::Number,  // token_type
            test2_input,        // value
            1,                  // line
            1,                  // start_col
            11,                  // end_col
        )
    );
}

