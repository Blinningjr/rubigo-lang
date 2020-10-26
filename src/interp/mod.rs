#![allow(dead_code)]

mod module_interp;
mod statements_interp;
mod expression_interp;
mod operations_interp;

pub use super::span::Span;

pub use super::parser::{
    ModualBody,
    statement,
    expressions,
    operations,
    Literal,
    Statement,
    TypeDecleration,
    statement::Function,
};

pub use module_interp::{
    InterpModule,
    InterpFuncEnv,
    InterpEnv,
    Value,
    Pointer,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    pub module: InterpModule,
}

impl Interpreter {
    pub fn interpret(ast: ModualBody) -> () {
        let mut interpreter: Interpreter = Interpreter{module: InterpModule::new()};

        interpreter.interpret_modual_body(ast);
    }

    fn interpret_modual_body(&mut self, mod_body: ModualBody) -> () {
        for stmt in mod_body.body.iter() {
            self.interpret_statement(stmt.clone())
        }
    }

    fn get_pointer(&mut self, identifier: String) -> Value {
        let (func_id, env_id, value_id) = self.module.search_value_location(identifier);
        return Value::Pointer(Pointer{
            func_id: func_id,
            env_id: env_id,
            value_id: value_id,
        });
    }

    fn get_variable(&mut self, identifier: String) -> Value {
        return self.module.get_variable(identifier);
    }

    fn get_value(&mut self, pointer: Pointer) -> Value {
       return self.get_value(pointer); 
    }

    fn get_function(&mut self, ident: String) -> Function {
        return self.module.get_function(ident);
    }

    fn get_current_func_id(& self) -> Option<usize> {
        return match self.module.func_envs.len() {
            0 => None,
            n => Some(n-1),
        }; 
    }

    fn get_current_env_id(& self) -> usize {
        return self.module.envs.len();
    }
}

