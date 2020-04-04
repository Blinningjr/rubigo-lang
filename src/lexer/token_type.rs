/**
 * All the different token types.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Fn,
    While,
    If,
    Else,
    Let,
    Return,
    
    EndExpression,

    Mut,
    Borrow,
   
    NewLine,
    Space,

    Ident,
    Type,
    Boolean,
    Number,
    Op,

    String,

    BodyStart,
    BodyEnd,
    ParenthesisStart,
    ParenthesisEnd,

    TypeDec,

    FnType,
}

