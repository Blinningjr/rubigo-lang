use super::{
    Expression,
    Literal,
};


/**
 * Defines Ast in Rubigo.
 */
pub enum Ast {
    Expression(Expression),
    Literal(Literal),
}
