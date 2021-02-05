#![allow(dead_code)]

mod value_borrow;
mod environment_borrow;
mod module_borrow;
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

pub use module_borrow::BorrowModule;

pub use environment_borrow::{
    BorrowFunction,
    BorrowEnvironment,
    BorrowVariable,
};


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowChecker {
    pub error_handler: ErrorHandler,
    pub module: BorrowModule,

    pub current_mod_env: usize,
    pub current_env: usize,
    pub current_func: Option<usize>,
}

impl BorrowChecker {
    pub fn check(ast: ModualBody, print_errors: bool) -> BorrowChecker {
        let mut checker: BorrowChecker = BorrowChecker{
            error_handler: ErrorHandler::new(true),
            module: BorrowModule::new(),
           
            current_mod_env: 0, // The current env id of the modual envs.
            current_env: 0,
            current_func: None,
        };

        //let mut print_func: Function = Function::create_dummy();
        //print_func.identifier = Span::new("print".to_string(), 0, 0);
        //print_func.parameters.push(Parameter{
        //                            mutable: None,
        //                            identifier: Span::new("DUMMY".to_string(), 0, 0), 
        //                            type_dec: TypeDecleration{borrow: false,
        //                                mutable: false,
        //                                r#type: Span::new(" ANY".to_string(), 0, 0)}});
        //checker.check_function(print_func);

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
    
//    fn create_error(&mut self, message: String) -> () {
//        let error: Error = Error::Error(message.clone());
//
//        self.error_handler.add(error);
//    }

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
    
    fn get_environment(&mut self) -> &mut BorrowEnvironment {
        return match self.current_func {
            Some(id) => &mut self.module.mod_funcs[id].environments.envs[self.current_env],
            None => &mut self.module.mod_envs.envs[self.current_env],
        };
    }

    fn get_variable(&mut self, ident: String, original: Span<String>) -> Option<(Option<usize>, usize, BorrowVariable)> { 
        match self.module.get_variable(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Some((func_id, env_id, val)) => return Some((func_id, env_id, val.clone())),
            None => return None,
        };
    }

    fn add_variable(&mut self, original: Span<String>, ident: Span<String>, mutable: bool, pointer: (usize, usize)) -> () {
        let borrow_var: BorrowVariable = BorrowVariable{
            original: original.clone(),
            ident: ident.clone(),

            pointer: pointer,
            mutable: mutable,
        };

        let result: Option<BorrowVariable> = self.get_environment().set_variable(borrow_var);
    }

    fn update_variable(&mut self, ident: String, deref: bool, value: BorrowValue) {
        self.
    }


    fn store_value(&mut self, value: BorrowValue, mutable: bool) -> (usize, usize) {
        self.get_environment().set_value(value, mutable)
    }
    

    fn create_body(&mut self) -> () {
        match self.current_func {
            Some(id) => self.current_env = self.module.mod_funcs[id].create_env(self.current_env),
            None => {
                let new_id: usize = self.module.mod_envs.envs.len();
                let current_id: usize = self.current_env;
                self.module.mod_envs.envs.push(BorrowEnvironment::new(new_id, Some(current_id)));
                self.current_env = new_id;
                self.current_mod_env = new_id;
            },
        };
    }
    
//    fn new_function_env(&mut self, function: Function, parameters: Vec<(bool, Type)>, return_type: Option<Type>) -> () {
//        let _prev_func: Option<usize> = self.current_func;
//        let new_func: usize = self.module.mod_funcs.len();
//
//        let type_func: BorrowFunction = BorrowFunction::new(function);
// 
//        self.module.mod_funcs.push(type_func);
//        
//        self.current_env = 0;
//        self.current_func = Option::Some(new_func);
//    }

    fn get_location(& self) -> (Option<usize>, usize) {
        return (self.current_func, self.current_env);
    }

//    fn check_borrow_scope(&mut self, let_location: (Option<usize>, usize), borrow_location: (Option<usize>, usize), ident: Span<String>, original: Span<String>) -> () {
//        if self.check_borrow_func_scope(let_location.0, borrow_location.0, ident.clone(), original.clone()) {
//            if let_location.1 < borrow_location.1 {
//                self.create_borrow_error(ErrorLevel::Error, 
//                                       format!("Variable {} lives longer then borrowed value.",
//                                               ident.get_fragment()),
//                                       original,
//                                       ident.get_line(),
//                                       ident.get_offset());   
//            }
//        }
//    }
//
//    // Returns true if let_func is a function in a equal function env then borrow_func.
//    fn check_borrow_func_scope(&mut self, let_func: Option<usize>, borrow_func: Option<usize>, ident: Span<String>, original: Span<String>) -> bool {
//        match (let_func, borrow_func) {
//            (Some(f_val), Some(b_val)) => {
//                if f_val == b_val {
//                    return true;
//                } else if f_val > b_val {
//                    return false;
//                }
//            },
//            (None, None) => return true,
//            (Some(_), None) => return false,
//            _ => (),
//        };
//        self.create_borrow_error(ErrorLevel::Error, 
//                               format!("Variable {} lives longer then borrowed value.",
//                                       ident.get_fragment()),
//                               original,
//                               ident.get_line(),
//                               ident.get_offset());
//        return false;
//    }
}

