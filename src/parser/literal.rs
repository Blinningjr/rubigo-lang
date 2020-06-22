use super::{
    Token,
    TokenType,
    TokenHandler,
    Atom,
    Span,
};


/**
 * All literals defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I32(i32),
    F32(f32),
    Bool(bool),
    Char(char),
    String(String),
}


/**
 * Checks if token is literal.
 */
pub fn is_literal(token: & Token) -> bool {
    return match token.get_type() { 
        TokenType::String => true,
        TokenType::Boolean => true,
        TokenType::FloatNumber => true,
        TokenType::Number => true,
        TokenType::Char => true,
        _ => false,
    };
}


/**
 * Parse any Literal.
 */
pub fn parse_literal(token_handler: &mut TokenHandler,
                     token: & Token) -> Span<Atom> {
    return match token.get_type() {
        TokenType::String => parse_string(token_handler, & token),
        TokenType::Boolean => parse_boolean(& token),
        TokenType::FloatNumber => parse_float_number(& token),
        TokenType::Number => parse_number(& token),
        TokenType::Char => parse_char(& token),
        _ => panic!("Syntax error: Expected literal."),
    };
}


/**
 * Parses tokens untill the end sting is found, into a literal String.
 */
pub fn parse_string(token_handler: &mut TokenHandler, start_string_token: & Token) -> Span<Atom> {
    match start_string_token.get_type() {
        TokenType::String => {
            let mut string: String = "".to_string();
            while token_handler.hungry() {
                let token: Token = token_handler.next_token(false).unwrap();
                match token.get_type() {
                    TokenType::String => {
                        return Span::new(
                            Atom::Literal(Literal::String(string)),
                            start_string_token.get_line(),
                            start_string_token.get_offset()
                        );
                    },
                    _ => string.push_str(& token.get_value()),
                };
            }
            panic!("Syntax error: Expected \"");
        },
        _ => panic!("Syntax error: Expected \""),
    };
}


/**
 * Parses a token of type Boolean into literal Bool.
 */
pub fn parse_boolean(token: & Token) -> Span<Atom> {
    match token.get_type() {
        TokenType::Boolean => {
            return match token.get_value().parse::<bool>() {
                Ok(val) => Span::new(
                    Atom::Literal(Literal::Bool(val)),
                    token.get_line(),
                    token.get_offset()
                ),
                Err(_e) => panic!("Syntax error: could not parse bool"),
            };
        },
        _ => panic!("Syntax error: expected boolean"),
    };
}


/**
 * Parses token of type FloatNumber into literal f32.
 */
pub fn parse_float_number(token: & Token) -> Span<Atom> {
    match token.get_type() {
        TokenType::FloatNumber => {
            return match token.get_value().parse::<f32>() {
                Ok(val) => Span::new(
                    Atom::Literal(Literal::F32(val)),
                    token.get_line(),
                    token.get_offset()
                ),
                Err(_e) => panic!("Syntax error: could not parse f32"),
            }; 
        },
        _ => panic!("Syntax error: could not parse f32"),
    };
}


/**
 * Parses token of type Number into literal i32.
 */
pub fn parse_number(token: & Token) -> Span<Atom> {
    match token.get_type() {
        TokenType::Number => {
            return match token.get_value().parse::<i32>() {
                Ok(val) => Span::new(
                    Atom::Literal(Literal::I32(val)),
                    token.get_line(),
                    token.get_offset()
                ),
                Err(_e) => panic!("Syntax error: could not parse i32"),
            }; 
        },
        _ => panic!("Syntax error: could not parse i32"),
    };
}


/**
 * Parses token of type Char into literal char.
 */
pub fn parse_char(token: & Token) -> Span<Atom> {
    match token.get_type() {
        TokenType::Char => {
            return match token.get_value().parse::<char>() {
                Ok(val) => Span::new(
                    Atom::Literal(Literal::Char(val)),
                    token.get_line(),
                    token.get_offset()
                ),
                Err(_e) => panic!("Syntax error: could not parse Char"),
            }; 
        },
        _ => panic!("Syntax error: could not parse Char"),
    };
}

