use super::{
    Parser,
    TokenType,
    Token,
    ErrorLevel,
    Span,
};


/**
 * Defines Type Declaration in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    pub borrow: bool,
    pub mutable: bool,
    pub r#type: Span<String>,
}


impl Parser {
    /**
     * Parse Type Declaration.
     */
    pub(super) fn parse_type_decleration(&mut self) -> TypeDecleration {
        let mut borrow: bool = false;
        let mut mutable: bool = false;

        if self.is_tokentype(TokenType::Borrow) {
            let _borrow: Token = self.next_token();
            borrow = true;
        }

        if self.is_tokentype(TokenType::Mut) { 
            let _mutable: Token = self.next_token();
            mutable = true;
        }

        match self.peak().get_type() {
            TokenType::Identifier => (),
            TokenType::Ti32 => (),
            TokenType::Tf32 => (),
            TokenType::TBool => (),
            TokenType::TChar=> (),
            TokenType::TString => (),
            TokenType::ParenthesisStart => {
                let (value, token): (String, Token) = self.parse_empty();
                return TypeDecleration {
                    borrow: borrow,
                    mutable: mutable,
                    r#type: self.create_span(value, & token),
                }; 
            },
            _ => {
                self.create_error(ErrorLevel::Error, "Expected a Type".to_string());
            },
        };
        
        let token: Token = self.next_token();

        return TypeDecleration {
            borrow: borrow,
            mutable: mutable,
            r#type: self.create_span(token.get_value(), & token),
        }; 
    }


    fn parse_empty(&mut self) -> (String, Token) {
        let start: Token = self.next_token();
        match self.peak().get_type() {
            TokenType::ParenthesisEnd => {
                let _end: Token = self.next_token();
                return ("".to_string(), start.clone());
            },
            _ => return ("".to_string(), start.clone()),
        }
    }
}

