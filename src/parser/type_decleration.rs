use super::{
    Type,
    Span,
    TokenType,
    TokenHandler,
    Token,
    parse_type,
};


/**
 * Defines Type Decleration in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    pub borrow: Option<Span<String>>,
    pub mutable: Option<Span<String>>,
    pub r#type: Span<Type>,
}


/**
 * Parse Type Decleration
 */
pub fn parse_type_decleration(token_handler: &mut TokenHandler,
                          token_1: & Token) -> TypeDecleration {
    match token_1.get_type() {
        TokenType::Borrow => {
            let borrow: Option<Span<String>> = Option::Some(Span::new(
                token_1.get_value(),
                token_1.get_line(),
                token_1.get_offset()));
            let token_2: Token = token_handler.next_token(true).unwrap();
            
            match token_2.get_type() {
                TokenType::Mut => {
                    let token_3: Token = token_handler.next_token(true).unwrap();
                    
                    return TypeDecleration{
                        borrow: borrow,
                        mutable: Option::Some(Span::new(
                            token_2.get_value(),
                            token_2.get_line(),
                            token_2.get_offset())),
                        r#type: parse_type(& token_3),
                    };
                },
                _ => {
                    return TypeDecleration{
                        borrow: borrow,
                        mutable: Option::None,
                        r#type: parse_type(& token_2),
                    };
                },
            }
        },
        TokenType::Mut => {
            let token_2: Token = token_handler.next_token(true).unwrap();
            
            return TypeDecleration{
                borrow: Option::None,
                mutable: Option::Some(Span::new(
                    token_1.get_value(),
                    token_1.get_line(),
                    token_1.get_offset())),
                r#type: parse_type(& token_2),
            };
        },
        _ => {
            return TypeDecleration{
                borrow: Option::None,
                mutable: Option::None,
                r#type: parse_type(& token_1),
            };
        },
    }
}

