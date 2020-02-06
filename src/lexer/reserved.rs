use super::{
    TokenType,
    TokenHandler,
    fsm_ident,
};


/**
 * Checks if ident equals to one of the reserved ident:s.
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
 * Checks symbols and preforms the related operation.
 * Returns true if operations is implemented for the symbol.
 */
pub fn check_symbols(token_handler: &mut TokenHandler, ch: char, look_a_head: char) -> bool {
    let mut result: bool = true;
    let mut input: String = String::from("");
    input.push(ch);
    input.push(look_a_head);

    match &input[..] {
        "->" => tokenize_symbols(token_handler, TokenType::FnType),
        "()" => tokenize_symbols(token_handler, TokenType::Type),

        "!=" => tokenize_symbols(token_handler, TokenType::Op),
        "==" => tokenize_symbols(token_handler, TokenType::Op),
        "+=" => tokenize_symbols(token_handler, TokenType::Op),
        "-=" => tokenize_symbols(token_handler, TokenType::Op),
        ">=" => tokenize_symbols(token_handler, TokenType::Op),
        "<=" => tokenize_symbols(token_handler, TokenType::Op),

        "&&" => tokenize_symbols(token_handler, TokenType::Op),
        "||" => tokenize_symbols(token_handler, TokenType::Op),

        _ => result = false,
    };
    return result;
}


/**
 * Checks symbol and preforms the related operation.
 * Returns true if operations is implemented for the symbol.
 */
pub fn check_symbol(token_handler: &mut TokenHandler, ch: char) -> bool {
    let mut result: bool = true;
    match ch {
        ' ' => token_handler.discard(),
        '\n' => token_handler.discard(),

        ';' => tokenize_simple_symbol(token_handler, TokenType::EndExpression),
        ':' => tokenize_simple_symbol(token_handler, TokenType::TypeDec),
        '"' => tokenize_simple_symbol(token_handler, TokenType::String),
        '{' => tokenize_simple_symbol(token_handler, TokenType::BodyStart),
        '}' => tokenize_simple_symbol(token_handler, TokenType::BodyEnd),
        '(' => tokenize_simple_symbol(token_handler, TokenType::ParenthesisStart),
        ')' => tokenize_simple_symbol(token_handler, TokenType::ParenthesisEnd),

        '&' => tokenize_simple_symbol(token_handler, TokenType::Borrow),

        '=' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '!' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '+' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '-' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '/' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '*' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '%' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '<' => tokenize_simple_symbol(token_handler, TokenType::Op),
        '>' => tokenize_simple_symbol(token_handler, TokenType::Op),
        
        '_' => {
            token_handler.consume();
            fsm_ident(token_handler);
        },
        _ => result = false,
    };
    return result;
}


/**
 * Consumes one char and makes a token of token type token_type.
 */
fn tokenize_simple_symbol(token_handler: &mut TokenHandler, token_type: TokenType) {
    token_handler.consume();
    token_handler.next_token(token_type);
}


/**
 * Consumes two chars and makes a token of token type token_type.
 */
fn tokenize_symbols(token_handler: &mut TokenHandler, token_type: TokenType) {
    token_handler.consume();
    tokenize_simple_symbol(token_handler, token_type);
}
