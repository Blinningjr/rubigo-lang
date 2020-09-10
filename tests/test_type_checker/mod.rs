#![allow(dead_code)]

#[path = "../../src/span.rs"]
mod span;

#[path = "../../src/error.rs"]
mod error;

#[path = "../../src/lexer/mod.rs"]
mod lexer;

#[path = "../../src/parser/mod.rs"]
mod parser;

#[path = "../../src/type_checker/mod.rs"]
pub mod type_checker;

mod type_errors;
mod statement;

pub use error::ErrorHandler;
pub use parser::Parser;
pub use type_checker::TypeChecker;

/**
 * Runs Type check on string. 
 */
pub fn type_check_string(input: String) -> TypeChecker {
    return TypeChecker::type_check(Parser::parse(input, true), false);
}

pub fn get_num_of_errors(type_checker: TypeChecker) -> usize {
    return type_checker.error_handler.get_num_errors();
}

pub fn check_no_errors(type_checker: TypeChecker) -> bool {
    return type_checker.error_handler.get_num_errors() == 0; 
}

