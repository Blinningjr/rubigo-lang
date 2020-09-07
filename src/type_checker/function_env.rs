#![allow(dead_code)]

pub use super::{
    Span,
    environment::Environment,
    statement::Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEnv {
    pub function: Function,
    pub environment: Environment, 
}


impl FunctionEnv {
    pub fn new(environment_id: usize, previus_env_id: Option<usize>, function: Function) -> FunctionEnv{
        return FunctionEnv{
            function: function,
            environment: Environment::new(environment_id, previus_env_id),
        };
    }

    pub fn add_function(&mut self, identifier: Span<String>, env_id: usize) -> bool {
        return self.environment.add_function(identifier, env_id);
    }

    pub fn lookup_function_id(&mut self, identifier: String) -> Result<usize, String> {
        if self.function.identifier.get_fragment() == identifier {
            return Ok(self.environment.environment_id);
        }
        return self.environment.lookup_function(identifier);
    }

    pub fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> bool {
        return self.environment.add_variable(identifier, r#type);
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<String, String> {
        return self.environment.lookup_variable(identifier); 
    }
}

