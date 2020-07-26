use super::{
    Parser,
    TokenType,
    Token,
    ErrorLevel,
    Error,
    SyntaxError,
};


/**
 * Defines Type Decleration in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    pub borrow: bool,
    pub mutable: bool,
    pub r#type: String,
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
                let err_token: Token = self.peak();
                self.error_handler.add(Error::SyntaxError(SyntaxError {
                    level: ErrorLevel::Error,
                    message: "Expected Type.".to_string(),
                    line: err_token.get_line(),
                    offset: err_token.get_offset(),
                }));

            },
        };
        
        let token: Token = self.next_token();

        return TypeDecleration {
            borrow: borrow,
            mutable: mutable,
            r#type: token.get_value(),
        }; 
    }
}

