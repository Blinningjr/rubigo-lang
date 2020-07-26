use super::{
    Parser,
    Token,
    TokenType,
    ErrorLevel,
    Error,
    SyntaxError,
};


/**
 * All literals defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I32(i32),
    F32(f32),
    Bool(bool),
    Char(char),
    String(String),
    Dummy,
}


impl Parser {
    /**
     * Parse literal.
     */
    pub(super) fn parse_literal(&mut self) -> Literal {
        if self.is_tokentype(TokenType::Number) {
            return self.parse_i32();

        } else if self.is_tokentype(TokenType::FloatNumber) {
            return self.parse_f32();

        } else if self.is_tokentype(TokenType::Boolean) {
            return self.parse_bool();

        } else if self.is_tokentype(TokenType::Char) {
            return self.parse_char();

        } else if self.is_tokentype(TokenType::String) {
            return self.parse_string();

        } else {
            let original_start: usize = self.get_original_start() - 1;

            let err_token: Token = self.peak();
            let code: String = self.get_original(original_start);
            
            self.error_handler.add(Error::SyntaxError(SyntaxError {
                level: ErrorLevel::Error,
                message: "Expected Literal.".to_string(),
                code: code, 
                line: err_token.get_line(),
                offset: err_token.get_offset(),
            }));
            return Literal::Dummy; 
        }
    }


    /**
     * Parse i32.
     */
    fn parse_i32(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::I32(token.get_value().parse::<i32>().unwrap());
    }


    /**
     * Parse f32.
     */
    fn parse_f32(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::F32(token.get_value().parse::<f32>().unwrap());
    }


    /**
     * Parse bool.
     */
    fn parse_bool(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::Bool(token.get_value().parse::<bool>().unwrap());
    }


    /**
     * Parse char.
     */
    fn parse_char(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::Char(token.get_value().parse::<char>().unwrap());
    }


    /**
     * Parse string.
     */
    fn parse_string(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::String(token.get_value());
    }

    
    /**
     * Checks if token is literal.
     */
    pub(super) fn is_literal(&mut self) -> bool {
        return match self.peak().get_type() { 
            TokenType::String => true,
            TokenType::Boolean => true,
            TokenType::FloatNumber => true,
            TokenType::Number => true,
            TokenType::Char => true,
            _ => false,
        };
    }
}

