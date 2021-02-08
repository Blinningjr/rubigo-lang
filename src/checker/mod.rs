#![allow(dead_code)]

mod r#type;
mod environment_type;
mod module_type;
mod expressions_type;
mod operations_type;
mod statements_type;

pub use super::span::Span;

pub use super::error::{
    ErrorLevel,
    Error,
    ErrorHandler,
    TypeError,
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

pub use module_type::TypeModule;

pub use environment_type::{
    TypeFunction,
    TypeEnvironment,
    TypeVariable,
};

pub use r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Checker {
    pub error_handler: ErrorHandler,
    pub module: TypeModule,

    pub current_mod_env: usize,
    pub current_env: usize,
    pub current_func: Option<usize>,
}

impl Checker {
    pub fn check(ast: ModualBody, print_errors: bool) -> Checker {
        let mut checker: Checker = Checker{
            error_handler: ErrorHandler::new(true),
            module: TypeModule::new(),
           
            current_mod_env: 0, // The current env id of the modual envs.
            current_env: 0,
            current_func: None,
        };

        let mut print_func: Function = Function::create_dummy();
        print_func.identifier = Span::new("print".to_string(), 0, 0);
        print_func.parameters.push(Parameter{
                                    mutable: None,
                                    identifier: Span::new("DUMMY".to_string(), 0, 0), 
                                    type_dec: TypeDecleration{borrow: false,
                                        mutable: false,
                                        r#type: Span::new(" ANY".to_string(), 0, 0)}});
        checker.check_function(print_func);

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
    
    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::Error(message.clone());

        self.error_handler.add(error);
    }

    fn create_type_error(&mut self, level: ErrorLevel, message: String, code: Span<String>, line: usize, offset: usize) -> () {
        let error: Error = Error::TypeError(TypeError {
            level: level,
            message: message.clone(),
            code: code,
            line: line,
            offset: offset,
        });

        self.error_handler.add(error);
    }


    fn create_borrow_error(&mut self, level: ErrorLevel, message: String, code: Span<String>, line: usize, offset: usize) -> () {
        let _error: Error = Error::BorrowError(BorrowError {
            level: level,
            message: message.clone(),
            code: code,
            line: line,
            offset: offset,
        });

//        self.error_handler.add(error);  TODO: Remove all the borrow checking 
    }
    
    fn get_function(&mut self, ident: String, original: Span<String>) -> Option<TypeFunction> {
        match self.module.get_function_id(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Some(id) => return Some(self.module.mod_funcs[id].clone()),
            None => {
                self.create_type_error(ErrorLevel::Error,
                                       format!("Function {} out of scope", ident),
                                       original.clone(),
                                       original.get_line(),
                                       original.get_offset());
                return None;
            },
        };
    }

    fn get_environment(&mut self) -> &mut TypeEnvironment {
        return match self.current_func {
            Some(id) => &mut self.module.mod_funcs[id].environments.envs[self.current_env],
            None => &mut self.module.mod_envs.envs[self.current_env],
        };
    }

    fn get_variable(&mut self, ident: String, original: Span<String>) -> Option<(Option<usize>, usize, TypeVariable)> { 
        match self.module.get_variable(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Some((func_id, env_id, val)) => return Some((func_id, env_id, val.clone())),
            None =>{
                self.create_type_error(ErrorLevel::Error,
                                       format!("Variable {} out of scope", ident),
                                       original.clone(),
                                       original.get_line(),
                                       original.get_offset());
                return None;
            },
        };
    }

    fn add_variable(&mut self, original: Span<String>, ident: Span<String>, mutable: bool, r#type: Type) -> () {
        let type_var: TypeVariable = TypeVariable{
            original: original.clone(),
            ident: ident.clone(),

            mutable: mutable,
            r#type: r#type.clone(),

            num_borrows: 0,
            borrowed_as_mut: false,
        };

        let result: Option<TypeVariable> = self.get_environment().set_variable(type_var);

        match result {
            Some(var) => {
                self.create_type_error(ErrorLevel::Error,
                                       format!("Variable {} already deleared at line {}", ident.get_fragment(), var.ident.get_fragment()),
                                       original,
                                       ident.get_line(),
                                       ident.get_offset());
            },
            None => (),
        }; 
    }
    

    fn create_body(&mut self) -> () {
        match self.current_func {
            Some(id) => self.current_env = self.module.mod_funcs[id].create_env(self.current_env),
            None => {
                let new_id: usize = self.module.mod_envs.envs.len();
                let current_id: usize = self.current_env;
                self.module.mod_envs.envs.push(TypeEnvironment::new(new_id, Some(current_id)));
                self.current_env = new_id;
                self.current_mod_env = new_id;
            },
        };
    }
    
    fn new_function_env(&mut self, function: Function, parameters: Vec<(bool, Type)>, return_type: Option<Type>) -> () {
        let _prev_func: Option<usize> = self.current_func;
        let new_func: usize = self.module.mod_funcs.len();

        let type_func: TypeFunction = TypeFunction::new(function, parameters, return_type);

        self.get_environment().set_function(type_func.clone());
       
        self.module.mod_funcs.push(type_func);
        
        self.current_env = 0;
        self.current_func = Option::Some(new_func);
    }


    fn add_borrow_as_mut(&mut self, ident: String, original: Span<String>) -> () {
        match self.module.add_borrow_as_mut(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Ok(found) => {
                if !found {
                    self.create_type_error(ErrorLevel::Error,
                                           format!("variable {} out of scope", ident),
                                           original.clone(),
                                           original.get_line(),
                                           original.get_offset(),);
                }
            },
            Err(_var) => {
               // self.create_type_error(ErrorLevel::Error,
               //                        format!("Can't borrow {} as mutable", ident),
               //                        original.clone(),
               //                        original.get_line(),
               //                        original.get_offset(),);
            },
        };
    }
    
    fn add_borrow(&mut self, ident: String, original: Span<String>) -> () {
        match self.module.add_borrow(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Ok(found) => {
                if !found {
                    self.create_type_error(ErrorLevel::Error,
                                           format!("variable {} out of scope", ident),
                                           original.clone(),
                                           original.get_line(),
                                           original.get_offset(),);
                }
            },
            Err(_var) => {
                //self.create_type_error(ErrorLevel::Error,
                //                       format!("Can't borrow {}", ident),
                //                       original.clone(),
                //                       original.get_line(),
                //                       original.get_offset(),);
            },
        };
    }

    fn remove_borrow_as_mut(&mut self, ident: String) -> () {
        self.module.remove_borrow_as_mut(ident,
                                         self.current_func,
                                         self.current_env,
                                         self.current_mod_env);
    }
    
    fn remove_borrow(&mut self, ident: String) -> () {
        self.module.remove_borrow(ident,
                                         self.current_func,
                                         self.current_env,
                                         self.current_mod_env);
    }


    fn remove_all_used_in_current_env(&mut self) -> () {
        let env = match self.current_func {
            Some(id) => self.module.mod_funcs[id].environments.envs[self.current_env].clone(),
            None => self.module.mod_envs.envs[self.current_env].clone(),
        };
        
        for (_, var) in &env.variables {
            match var.r#type.ident.clone() {
                Some(ident) => {
                    if var.r#type.borrow && var.r#type.mutable {
                        self.remove_borrow_as_mut(ident);
                    } else if var.r#type.borrow {
                        self.remove_borrow(ident);
                    }
                },
                None => (),
            }
        }
    }

    fn check_if_all_bodies_return(&mut self) -> () {
        match self.current_func {
            Some(id) => {
                if !self.module.mod_funcs[id].check_if_all_bodies_return() {
                    let function: Function = self.module.mod_funcs[id].og_func.clone();
                    self.create_type_error(ErrorLevel::Error, 
                                           format!("Function {} might not return a value",
                                                   function.identifier.get_fragment()),
                                           function.original,
                                           function.identifier.get_line(),
                                           function.identifier.get_offset());
                }
            },
            None => panic!("Fatal error in type checker!!!"),
        }; 
    }

    fn set_is_if_body(&mut self) -> () {
        match self.current_func {
            Some(id) => {
                let env_id: usize = self.module.mod_funcs[id].environments.envs.len() - 1;
                self.module.mod_funcs[id].environments.envs[env_id].if_body = true;
            },
            None => (),
        };
    }
    
    fn set_is_else_body(&mut self) -> () {
        match self.current_func {
            Some(id) => {
                let env_id: usize = self.module.mod_funcs[id].environments.envs.len() - 1;
                self.module.mod_funcs[id].environments.envs[env_id].else_body = true;
            },
            None => (),
        };
    }

    fn get_location(& self) -> (Option<usize>, usize) {
        return (self.current_func, self.current_env);
    }

    fn check_borrow_scope(&mut self, let_location: (Option<usize>, usize), borrow_location: (Option<usize>, usize), ident: Span<String>, original: Span<String>) -> () {
        if self.check_borrow_func_scope(let_location.0, borrow_location.0, ident.clone(), original.clone()) {
            if let_location.1 < borrow_location.1 {
                self.create_borrow_error(ErrorLevel::Error, 
                                       format!("Variable {} lives longer then borrowed value.",
                                               ident.get_fragment()),
                                       original,
                                       ident.get_line(),
                                       ident.get_offset());   
            }
        }
    }

    // Returns true if let_func is a function in a equal function env then borrow_func.
    fn check_borrow_func_scope(&mut self, let_func: Option<usize>, borrow_func: Option<usize>, ident: Span<String>, original: Span<String>) -> bool {
        match (let_func, borrow_func) {
            (Some(f_val), Some(b_val)) => {
                if f_val == b_val {
                    return true;
                } else if f_val > b_val {
                    return false;
                }
            },
            (None, None) => return true,
            (Some(_), None) => return false,
            _ => (),
        };
        self.create_borrow_error(ErrorLevel::Error, 
                               format!("Variable {} lives longer then borrowed value.",
                                       ident.get_fragment()),
                               original,
                               ident.get_line(),
                               ident.get_offset());
        return false;
    }
}

