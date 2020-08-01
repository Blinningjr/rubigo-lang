use super::{
    Parser,
    TokenType,
    Token,
    ErrorLevel,
    Span,
};


/**
 * Defines Type Decleration in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    pub borrow: bool,
    pub mutable: bool,
    pub r#type: Span<String>,
}


impl Parser {
    /**
     * Parse Type Decleration.
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
}

