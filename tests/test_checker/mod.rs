#![allow(dead_code)]

#[path = "../../src/span.rs"]
mod span;

#[path = "../../src/error.rs"]
mod error;

#[path = "../../src/lexer/mod.rs"]
mod lexer;

#[path = "../../src/parser/mod.rs"]
mod parser;

#[path = "../../src/checker/mod.rs"]
pub mod checker;

mod type_errors;
mod statement;
mod operation;

pub use error::ErrorHandler;
pub use parser::Parser;
pub use checker::Checker;

/**
 * Runs Type check on string. 
 */
pub fn type_check_string(input: String) -> Checker {
    return Checker::check(Parser::parse("TEST".to_string(), input, true), false);
}

pub fn get_num_of_errors(checker: Checker) -> usize {
    return checker.error_handler.get_num_errors();
}

pub fn check_no_errors(checker: Checker) -> bool {
    return checker.error_handler.get_num_errors() == 0; 
}

