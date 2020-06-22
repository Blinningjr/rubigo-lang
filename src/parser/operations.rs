use super::{
    Token,
    TokenType,
    Span,
    Atom,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
   ParenthesisStart,
   ParenthesisEnd,
   Not,
   Plus,
   Minus,
   Divition,
   Multiplication,
   Modilus,
   LessThen,
   GreaterThen,
   NotEqual,
   Equal,
   GreaterEqual,
   LessEqual,
   And,
   Or,
}


/**
 * Is this Needed?
 * Defines variable in Rubigo.
 */
pub struct Variable {
    ident: String,
}


/**
 * Checks if token is operation.
 */
pub fn is_operation(token: & Token) -> bool {
    return match token.get_type() {
        TokenType::ParenthesisStart => true,
        TokenType::ParenthesisEnd => true,
        TokenType::Not => true,
        TokenType::Plus => true,
        TokenType::Minus => true,
        TokenType::ForwardSlash => true,
        TokenType::Star => true,
        TokenType::Modilus => true,
        TokenType::LessThen => true,
        TokenType::GreaterThen => true,
        TokenType::NotEqual => true,
        TokenType::Equal => true,
        TokenType::GreaterEqual => true,
        TokenType::LessEqual => true,
        TokenType::And => true,
        TokenType::Or => true,
        _ => false,
    };
}


/**
 * Parsese operation.
 */
pub fn parse_operation(token: & Token) -> Span<Atom> {
    return Span::new(
        Atom::Operation(match token.get_type() {
            TokenType::ParenthesisStart => 
                Operation::ParenthesisStart,
            TokenType::ParenthesisEnd => 
                Operation::ParenthesisEnd,
            TokenType::Not => 
                Operation::Not,
            TokenType::Plus => 
                Operation::Plus,
            TokenType::Minus =>
                Operation::Minus,
            TokenType::ForwardSlash =>
                Operation::Divition,
            TokenType::Star =>
                Operation::Multiplication,
            TokenType::Modilus =>
                Operation::Modilus,
            TokenType::LessThen =>
                Operation::LessThen,
            TokenType::GreaterThen =>
                Operation::GreaterThen,
            TokenType::NotEqual =>
                Operation::NotEqual,
            TokenType::Equal =>
                Operation::Equal,
            TokenType::GreaterEqual =>
                Operation::GreaterEqual,
            TokenType::LessEqual =>
                Operation::LessEqual,
            TokenType::And =>
                Operation::And,
            TokenType::Or =>
                Operation::Or,
            _ => panic!("Syntax error: Expected operation."),
        }),
        token.get_line(),
        token.get_offset()
    );
}

