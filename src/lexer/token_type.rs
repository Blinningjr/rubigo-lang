/**
 * All the different token types.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,

    Fn,
    While,
    If,
    Else,
    Let,
    Return,
    
    SemiColon,

    Mut,
    Borrow,
   
    Identifier,
    
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


impl TokenType {
    pub fn revert(& self) -> String {
        return match self {
            TokenType::EOF => "EOF".to_string(),

            TokenType::Fn => "fn".to_string(),
            TokenType::While => "while".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::Let => "let".to_string(),
            TokenType::Return => "return".to_string(),
            
            TokenType::SemiColon => ";".to_string(),

            TokenType::Mut => "mut".to_string(),
            TokenType:: Borrow => "&".to_string(),

            TokenType:: Identifier => "Identifier".to_string(),
            TokenType:: TBool => "bool".to_string(),
            TokenType:: Ti32 => "i32".to_string(),
            TokenType:: Tf32 => "f32".to_string(),
            TokenType:: TChar => "char".to_string(),
            TokenType:: TString => "string".to_string(),

            TokenType:: Boolean => "Boolean".to_string(),
            TokenType:: Number => "Integer".to_string(),
            TokenType:: FloatNumber => "Float".to_string(),

            TokenType:: NotEqual => "!=".to_string(),
            TokenType:: Equal => "==".to_string(),
            TokenType:: PlusEquals => "+=".to_string(),
            TokenType:: MinusEquals => "-=".to_string(),
            TokenType:: GreaterEqual => ">=".to_string(),
            TokenType:: LessEqual => "<=".to_string(),
            TokenType:: And => "&&".to_string(),
            TokenType:: Or => "||".to_string(),

            TokenType:: Equals => "=".to_string(),
            TokenType:: Not => "!".to_string(),
            TokenType:: Plus => "+".to_string(),
            TokenType:: Minus => "-".to_string(),
            TokenType:: ForwardSlash => "/".to_string(),
            TokenType:: Star => "*".to_string(),
            TokenType:: Modilus => "%".to_string(),
            TokenType:: LessThen => "<".to_string(),
            TokenType:: GreaterThen => ">".to_string(),

            TokenType:: Char => "'".to_string(),
            TokenType:: String => "\"".to_string(),

            TokenType:: BodyStart => "{".to_string(),
            TokenType:: BodyEnd => "}".to_string(),
            TokenType:: ParenthesisStart => "(".to_string(),
            TokenType:: ParenthesisEnd => ")".to_string(),

            TokenType:: TypeDec => ":".to_string(),

            TokenType:: FnType => "->".to_string(),

            TokenType:: Comma => ",".to_string(),
        };
    }
}

