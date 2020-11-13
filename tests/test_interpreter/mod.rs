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

#[path = "../../src/interp/mod.rs"]
pub mod interpreter;

mod statements;
//mod operations;

pub use parser::{
    Parser, 
    ModualBody,
    literal::Literal,
    statement::Statement,
    Span,
    statement,
    Expression,
};

pub use checker::Checker;
pub use error::ErrorHandler;
pub use interpreter::{
    Interpreter,
    InterpEnv,
    InterpModule,
    Value,
};


pub fn interpret_statement(input: Statement) -> (Option<Value>, Interpreter) {
    let mut interpreter: Interpreter = Interpreter{module: InterpModule::new()};
    let value = interpreter.interpret_statement(input);
    return (value, interpreter);
}


