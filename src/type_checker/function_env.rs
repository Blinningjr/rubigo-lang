#![allow(dead_code)]

pub use super::{
    Span,
    environment::Environment,
    statement::Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEnv {
    pub id: usize,
    pub previus_id: Option<usize>,

    pub function: Function,
    pub environments: Vec<Environment>, 
}


impl FunctionEnv {
    pub fn new(id: usize, previus_id: Option<usize>, function: Function) -> FunctionEnv{
        // TODO: Add the function parameters as variables in the first environment.
        return FunctionEnv{
            id: id,
            previus_id: previus_id,

            function: function,
            environments: vec!(Environment::new(0, None)),
        };
    }

    pub fn add_function(&mut self, identifier: Span<String>, env_id: usize) -> bool {
        return self.environments[0].add_function(identifier, env_id);
    }

    pub fn lookup_function_id(&mut self, identifier: String) -> Result<usize, String> {
        if self.function.identifier.get_fragment() == identifier {
            return Ok(self.id);
        }
        return self.environments[0].lookup_function(identifier);
    }

    pub fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> bool {
        return self.environments[0].add_variable(identifier, r#type);
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<String, String> {
        return self.environments[0].lookup_variable(identifier); 
    }
}

