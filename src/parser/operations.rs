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

