use super::{
    Token,
    TokenType,
    Span,
};

/**
 * All types defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    F32,
    Bool,
    Char,
    String,
}


/**
 * Parses type token into Type.
 */
pub fn parse_type(token: & Token) -> Span<Type> {
    return match token.get_type() {
        TokenType::TBool => Span::new(
            Type::Bool,
            token.get_line(),
            token.get_offset()
        ),
        TokenType::Ti32 => Span::new(
            Type::I32,
            token.get_line(),
            token.get_offset()
        ),
        TokenType::Tf32 => Span::new(
            Type::F32,
            token.get_line(),
            token.get_offset()
        ),
        TokenType::TChar => Span::new(
            Type::Char,
            token.get_line(),
            token.get_offset()
        ),
        TokenType::TString => Span::new(
            Type::String,
            token.get_line(),
            token.get_offset()
        ),
        _ => panic!("Syntax error: expected a Type."),
    };
}

