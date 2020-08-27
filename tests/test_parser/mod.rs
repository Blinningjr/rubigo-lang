#![allow(dead_code)]

#[path = "../../src/span.rs"]
pub mod span;

#[path = "../../src/error.rs"]
pub mod error;

#[path = "../../src/lexer/mod.rs"]
pub mod lexer;

#[path = "../../src/parser/mod.rs"]
pub mod parser;

mod statement;

use parser::{
    Parser,
    Statement,
};


/**
 * parsers the string into statments, expressions and more.
 */
pub fn parse_string(input: String) -> Statement {
    return Parser::parse(input, false);
}

