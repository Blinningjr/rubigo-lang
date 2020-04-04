use super::TokenType;


/**
 * Lexer Token.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    start_col: usize,
    end_col: usize,
}


/**
 * Implments Token functions.
 */
impl Token {
    /**
     * Returns the type of the token. 
     */
    pub fn new(token_type: TokenType, value: String, line: usize, start_col: usize, end_col: usize) -> Token {
        Token{
            token_type: token_type,
            value: value,
            line: line,
            start_col: start_col,
            end_col: end_col,
        }
    }


    /**
     * Returns the type of the token. 
     */
    pub fn get_type(&self) -> TokenType {
        return self.token_type.clone();
    }
}

