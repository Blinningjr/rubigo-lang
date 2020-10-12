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
    
    fn get_function(& self, ident: String) -> TypeFunction {
        match self.module.get_function_id(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Some(id) => return self.module.mod_funcs[id].clone(),
            None => panic!("TODO: Add type error {}", ident),
        };
    }

    fn get_environment(&mut self) -> &mut TypeEnvironment {
        return match self.current_func {
            Some(id) => &mut self.module.mod_funcs[id].environments.envs[self.current_env],
            None => &mut self.module.mod_envs.envs[self.current_env],
        };
    }

    fn get_variable(&mut self, ident: String, original: Span<String>) -> (Option<usize>, usize, TypeVariable) { 
        match self.module.get_variable(ident.clone(), self.current_func, self.current_env, self.current_mod_env) {
            Some((func_id, env_id, val)) => return (func_id, env_id, val.clone()),
            None => panic!("TODO: add type error here {}", ident),
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
                panic!("TODO: add error");
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
        let prev_func: Option<usize> = self.current_func;
        let new_func: usize = self.module.mod_funcs.len();

        let type_func: TypeFunction = TypeFunction::new(function, parameters, return_type);

        self.get_environment().set_function(type_func.clone());
       
        self.module.mod_funcs.push(type_func);
        
        self.current_env = 0;
        self.current_func = Option::Some(new_func);
    }
}

