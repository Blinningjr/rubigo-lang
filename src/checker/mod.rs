#![allow(dead_code)]

mod r#type;
mod environment_type;
mod module_type;
mod literals_type;
mod expressions_type;
mod operations_type;
mod statements_type;

pub use super::span::Span;

pub use super::error::{
    ErrorLevel,
//    Error,
    ErrorHandler,
//    TypeError,
};

pub use super::parser::{
    statement,
    expressions,
    operations,
    literal,
//    TypeDecleration,
    ModualBody,
};

pub use module_type::TypeModule;

pub use r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Checker {
    pub error_handler: ErrorHandler,
    pub module: TypeModule,

    pub current_env: usize,
    pub current_func: Option<usize>,
}

impl Checker {
    pub fn check(ast: ModualBody, print_errors: bool) -> Checker {
        let mut checker: Checker = Checker{
            error_handler: ErrorHandler::new(true),
            module: TypeModule::new(),
            
            current_env: 0,
            current_func: None,
        };

        checker.check_modual_body(ast);
        
        if print_errors { 
            checker.error_handler.print_errors();
        }

        return checker;
    }

    fn check_modual_body(&mut self, mod_body: ModualBody) -> () {
        for stmt in mod_body.body.iter() {
            self.check_statement(stmt.clone());
        }
    }

}

