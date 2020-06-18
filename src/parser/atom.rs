use super ::{
    Literal,
    Operation,
    Span,
};


/**
 * Defines atoms in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Literal(Literal),
    Operations(Operation),
}

