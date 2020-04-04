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
        
        "bool" => TokenType::Type,
        "i32" => TokenType::Type,
        "f32" => TokenType::Type,
        "String" => TokenType::Type,

        _ => TokenType::Ident,
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
        "()" => Ok(TokenType::Type),

        "!=" => Ok(TokenType::Op),
        "==" => Ok(TokenType::Op),
        "+=" => Ok(TokenType::Op),
        "-=" => Ok(TokenType::Op),
        ">=" => Ok(TokenType::Op),
        "<=" => Ok(TokenType::Op),

        "&&" => Ok(TokenType::Op),
        "||" => Ok(TokenType::Op),

        _ => Err("Symbols are not reserved"),
    };
}


/**
 * Cheacks if symbole is reserved.
 * Returns token type or error.
 */
    pub fn check_symbol(ch: char) -> bool {
        return match ch {
            ' ' => Ok(TokenType::Space),
            '\n' => Ok(TokenType::NewLine),

            ';' => Ok(TokenType::EndExpression),
            ':' => Ok(TokenType::TypeDec),
            '"' => Ok(TokenType::String),
            '{' => Ok(TokenType::BodyStart),
            '}' => Ok(TokenType::BodyEnd),
            '(' => Ok(TokenType::ParenthesisStart),
            ')' => Ok(TokenType::ParenthesisEnd),

            '&' => Ok(TokenType::Borrow),

            '=' => Ok(TokenType::Op),
            '!' => Ok(TokenType::Op),
            '+' => Ok(TokenType::Op),
            '-' => Ok(TokenType::Op),
            '/' => Ok(TokenType::Op),
            '*' => Ok(TokenType::Op),
            '%' => Ok(TokenType::Op),
        '<' => Ok(TokenType::Op),
        '>' => Ok(TokenType::Op),
        
        _ => Err("Symbol is not reserved"),
    };
    return result;
}

