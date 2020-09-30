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

#[path = "../../src/interpreter/mod.rs"]
pub mod interpreter;

mod test_statements;

pub use parser::{
    Parser, 
    ModualBody,
    literal::Literal,
    statement::Statement,
    Span,   
};

pub use type_checker::TypeChecker;
pub use error::ErrorHandler;
pub use interpreter::{
    Interpreter,
    InterpEnv,
};


pub fn interpret_modual(input: String) -> (Literal, Interpreter) {
    let mut interpreter: Interpreter = Interpreter{
        error_handler: ErrorHandler::new(true),
         
        modual: TypeChecker::type_check(Parser::parse("TEST".to_string(), input, true), false).modual,
 
        envs: vec!(InterpEnv::new()),
        func_envs: vec!(),
    };
    let ast: ModualBody = interpreter.modual.ast.clone();

    let mut literal: Literal = Literal::Dummy;
    for stmt in ast.body.iter() {
        literal = interpreter.interpret_statement(stmt.clone());
    }

    return (literal, interpreter);
}


