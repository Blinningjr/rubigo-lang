use super::{
    Parser,
    TokenType,
    Token,
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
            let _borrow: Token = self.next_token(true);
            borrow = true;
        }

        if self.is_tokentype(TokenType::Mut) { 
            let _mutable: Token = self.next_token(true);
            mutable = true;
        }

        match self.lexer.peak(true).unwrap().get_type() {
            TokenType::Identifier => (),
            TokenType::Ti32 => (),
            TokenType::Tf32 => (),
            TokenType::TBool => (),
            TokenType::TChar=> (),
            TokenType::TString => (),
            _ => panic!("Expected Type \n{:#?}", self.lexer.peak(true).unwrap()),
        };
        
        let token: Token = self.next_token(true);

        self.empty_tokens();
        return TypeDecleration {
            borrow: borrow,
            mutable: mutable,
            r#type: token.get_value(),
        }; 
    }
}

