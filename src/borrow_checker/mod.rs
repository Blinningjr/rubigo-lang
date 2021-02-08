#![allow(dead_code)]

mod value_borrow;
mod environment_borrow;
mod expressions_borrow;
mod operations_borrow;
mod statements_borrow;

pub use super::span::Span;

pub use super::error::{
    ErrorLevel,
    Error,
    ErrorHandler,
    BorrowError,
};

pub use super::parser::{
    statement,
    expressions,
    operations,
    literal,
    TypeDecleration,
    ModualBody,

    statement::{
        Function,
        Parameter,
        Let,
    },
};

pub use value_borrow::BorrowValue;


pub use environment_borrow::{
    BorrowEnvironments,
    BorrowEnvironment,
    BorrowVariable,
};


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowChecker {
    pub error_handler: ErrorHandler,
    pub funcs: Vec<Function>,
}

impl BorrowChecker {
    pub fn check(ast: ModualBody, print_errors: bool) -> BorrowChecker {
        let mut checker: BorrowChecker = BorrowChecker{
            error_handler: ErrorHandler::new(true),
            funcs: Vec::new(),
        };

        let mut envs = checker.create_envs();


        checker.check_modual_body(ast, &mut envs);
        checker.check_functions();

        if print_errors { 
            checker.error_handler.print_errors();
        }

        return checker;
    }

    fn check_functions(&mut self) {
        if self.funcs.len() > 0 {
            let func = self.funcs.remove(0);
            self.check_function(func);
            self.check_functions();
        }
    }

    fn create_envs(&mut self) -> BorrowEnvironments {
        return BorrowEnvironments::new();
    }

    fn check_modual_body(&mut self, mod_body: ModualBody, envs: &mut BorrowEnvironments) -> () {
        for stmt in mod_body.body.iter() {
            self.check_statement(envs, stmt.clone());
        }
    }

    fn add_function(&mut self, function: Function) {
        self.funcs.push(function);    
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::Error(message.clone()); self.error_handler.add(error);
    }

    fn create_borrow_error(&mut self, level: ErrorLevel, message: String, code: Span<String>, line: usize, offset: usize) -> () {
        let error: Error = Error::BorrowError(BorrowError {
            level: level,
            message: message.clone(),
            code: code,
            line: line,
            offset: offset,
        });

        self.error_handler.add(error);
    }    
}

