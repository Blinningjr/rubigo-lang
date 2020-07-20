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
    
    TBool,
    Ti32,
    Tf32,
    TChar,
    TString,

    Boolean,
    Number,
    FloatNumber,

    NotEqual,
    Equal,
    PlusEquals,
    MinusEquals,
    GreaterEqual,
    LessEqual,
    And,
    Or,

    Equals,
    Not,
    Plus,
    Minus,
    ForwardSlash,
    Star,
    Modilus,
    LessThen,
    GreaterThen,

    Char,
    String,

    BodyStart,
    BodyEnd,
    ParenthesisStart,
    ParenthesisEnd,

    TypeDec,

    FnType,

    Comma,
}

