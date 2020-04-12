use super::TokenType;


/**
 * Lexer Token.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,      // The type of token.
    value: String,              // The raw value of the token.
    line: usize,                // The line the token appears
    offset: usize,              // The number of chars before the token on that lien.
}


/**
 * Implments Token functions.
 */
impl Token {
    /**
     * Returns the type of the token. 
     */
    pub fn new(token_type: TokenType, value: String, line: usize, offset: usize) -> Token {
        Token{
            token_type: token_type,
            value: value,
            line: line,
            offset: offset,
        }
    }


    /**
     * Returns the type of the token. 
     */
    pub fn get_type(&self) -> TokenType {
        return self.token_type.clone();
    }
}

