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


pub fn interpret_string(input: String) -> (Vec<Option<Value>>, Interpreter) {
    let ast: ModualBody = Parser::parse("TEST".to_string(), input, false);
    let mut result: Vec<Option<Value>> = vec!();
    let mut interpreter: Interpreter = Interpreter{module: InterpModule::new()};
    
    for stmt in ast.body.iter() {
        result.push(interpreter.interpret_statement(stmt.clone()));
    }
    return (result, interpreter);
}

