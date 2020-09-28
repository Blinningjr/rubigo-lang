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
    literal::Literal,
    statement::Statement,
    
};

pub use type_checker::TypeChecker;
pub use error::ErrorHandler;
pub use interpreter::{
    Interpreter,
    InterpEnv,
};



pub fn interpret_statement(input: String) -> Literal {
     let mut interpreter: Interpreter = Interpreter{
         error_handler: ErrorHandler::new(true),
         
         modual: TypeChecker::type_check(Parser::parse(input, true), false).modual,
 
         env: InterpEnv::new(),
         func_envs: vec!(),
     };
     let ast: Statement = interpreter.modual.ast.clone();
     
    return interpreter.interpret_statement(ast);;
}

