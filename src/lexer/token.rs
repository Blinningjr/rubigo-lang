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
     * Creates a new Token. 
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


    /**
     * Returns the value of the token.
     */
    pub fn get_value(&self) -> String {
        return self.value.clone();
    }


    /**
     * Returns the line in the file where the token value is taken from.
     */
    pub fn get_line(&self) -> usize {
        return self.line;
    }


    /**
     * Returns the offset from the begining of the line to the token value.
     */
    pub fn get_offset(&self) -> usize {
        return self.offset;
    }


    /**
     * Returns the end position of the token.
     */
    pub fn get_end_offset(&self) -> usize {
        return self.offset + self.value.len();
    }
}

