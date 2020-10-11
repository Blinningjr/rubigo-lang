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
//    TypeDecleration,
    ModualBody,

    statement::{
        Let,
    },
};

pub use module_type::TypeModule;

pub use environment_type::{
    TypeFunction,
    TypeEnvironment,
    TypeVarMem,
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
        match self.module.get_function_id(ident, self.current_func, self.current_env) {
            Some(id) => return self.module.mod_funcs[id].clone(),
            None => panic!("TODO: Add type error"),
        };
    }

    fn get_environment(&mut self) -> &mut TypeEnvironment {
        return match self.current_func {
            Some(id) => &mut self.module.mod_funcs[id].environments.envs[self.current_env],
            None => &mut self.module.mod_envs.envs[self.current_env],
        };
    }

    fn get_variable(&mut self, ident: String, original: Span<String>) -> (Option<usize>, usize, TypeVarMem) {
        
        match self.module.get_variable(ident, self.current_func, self.current_env) {
            Some((func_id, env_id, val)) => return (func_id, env_id, val.clone()),
            None => panic!("TODO: add type error here"),
        };
    }

    fn add_variable(&mut self, let_stmt: Let, r#type: Type) -> () {
        let type_var: TypeVariable = TypeVariable{
            og_var: let_stmt.clone(),

            mutable: let_stmt.mutable != None,
            r#type: r#type.clone(),

            num_borrows: 0,
            borrowed_as_mut: false,
        };

        let result: Option<TypeVarMem>;
        if r#type.borrow {
           let (func_id, env_id, ref_var) = self.get_variable(let_stmt.identifier.get_fragment(),
                                                              let_stmt.original.clone());
           result = self.get_environment().set_variable(TypeVarMem::Pointer(func_id, env_id, ref_var.get_ident(), type_var));
        } else {
            result = self.get_environment().set_variable(TypeVarMem::Var(type_var));
        } 

        match result {
            Some(var) => {
                panic!("TODO: add error");
            },
            None => (),
        }; 
    }
}

