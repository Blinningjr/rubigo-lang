use super::{
    Parser,
    Token,
    TokenType,
    ErrorLevel,
    Span,
};


/**
 * All literals defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I32(Span<i32>),
    F32(Span<f32>),
    Bool(Span<bool>),
    Char(Span<char>),
    String(Span<String>),
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
            self.create_error(ErrorLevel::Error, "Expected a Literal".to_string());
            return Literal::Dummy; 
        }
    }


    /**
     * Parse i32.
     */
    fn parse_i32(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::I32(self.create_span(token.get_value().parse::<i32>().unwrap(), & token));
    }


    /**
     * Parse f32.
     */
    fn parse_f32(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::F32(self.create_span(token.get_value().parse::<f32>().unwrap(), & token));
    }


    /**
     * Parse bool.
     */
    fn parse_bool(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::Bool(self.create_span(token.get_value().parse::<bool>().unwrap(), & token));
    }


    /**
     * Parse char.
     */
    fn parse_char(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::Char(self.create_span(token.get_value().parse::<char>().unwrap(), & token));
    }


    /**
     * Parse string.
     */
    fn parse_string(&mut self) -> Literal {
        let token: Token = self.next_token();
        return Literal::String(self.create_span(token.get_value(), & token));
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

