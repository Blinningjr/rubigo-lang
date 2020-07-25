use super::TokenType;


/**
 * Checks if ident equals to one of the reserved ident:s.
 * Returns a reserved token type or the ident token type.
 */
pub fn check_reserved_idents(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Fn,
        "while" => TokenType::While,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "let" => TokenType::Let,
        "return" => TokenType::Return,

        "mut" => TokenType::Mut,

        "true" => TokenType::Boolean,
        "false" => TokenType::Boolean,
        
        "bool" => TokenType::TBool,
        "i32" => TokenType::Ti32,
        "f32" => TokenType::Tf32,
        "Char" => TokenType::TChar,
        "String" => TokenType::TString,

        _ => TokenType::Identifier,
    }
}


/**
 * Checks if input is two reserved symboles.
 * Returns token type or error.
 */
pub fn check_symbols(ch: char, look_a_head: char) -> Result<TokenType, &'static str> {
    let mut input: String = String::from("");
    input.push(ch);
    input.push(look_a_head);

    return match &input[..] {
        "->" => Ok(TokenType::FnType),

        "!=" => Ok(TokenType::NotEqual),
        "==" => Ok(TokenType::Equal),
        "+=" => Ok(TokenType::PlusEquals),
        "-=" => Ok(TokenType::MinusEquals),
        ">=" => Ok(TokenType::GreaterEqual),
        "<=" => Ok(TokenType::LessEqual),

        "&&" => Ok(TokenType::And),
        "||" => Ok(TokenType::Or),

        _ => Err("Symbols are not reserved"),
    };
}


/**
 * Cheacks if symbole is reserved.
 * Returns token type or error.
 */
pub fn check_symbol(ch: char) -> Result<TokenType, &'static str> {
    return match ch {
        ' ' => Ok(TokenType::Space),
        '\n' => Ok(TokenType::NewLine),

        ';' => Ok(TokenType::SemiColon),
        ':' => Ok(TokenType::TypeDec),
        '\'' => Ok(TokenType::Char),
        '"' => Ok(TokenType::String),
        '{' => Ok(TokenType::BodyStart),
        '}' => Ok(TokenType::BodyEnd),
        '(' => Ok(TokenType::ParenthesisStart),
        ')' => Ok(TokenType::ParenthesisEnd),

        ',' => Ok(TokenType::Comma),

        '&' => Ok(TokenType::Borrow),

        '=' => Ok(TokenType::Equals),
        '!' => Ok(TokenType::Not),
        '+' => Ok(TokenType::Plus),
        '-' => Ok(TokenType::Minus),
        '/' => Ok(TokenType::ForwardSlash),
        '*' => Ok(TokenType::Star),
        '%' => Ok(TokenType::Modilus),
        '<' => Ok(TokenType::LessThen),
        '>' => Ok(TokenType::GreaterThen),
        
        _ => Err("Symbol is not reserved"),
    };
}

