#![allow(dead_code)]

pub use super::{
    Span,
    environment::Environment,
    statement::Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEnv {
    pub environment_id: usize,
    pub previus_env_id: usize,
    pub function: Option<Function>,
    pub environment: Environment, 
}


impl FunctionEnv {
    pub fn new(environment_id: usize, previus_env_id: usize) -> FunctionEnv{
        return FunctionEnv{
            environment_id: environment_id,
            previus_env_id: previus_env_id,
            function: None,
            environment: Environment::new(),
        };
    }

    pub fn add_function(&mut self, identifier: Span<String>, r#type: Span<String>) -> bool {
        return self.environment.add_function(identifier, r#type);
    }

    pub fn lookup_function(&mut self, identifier: String) -> Result<String, String> {
        match & self.function {
            Some(func) => {
                if func.identifier.get_fragment() == identifier {
                    return Ok(func.return_type.r#type.get_fragment());
                }
            },
            None => (),
        };
        return self.environment.lookup_function(identifier);
    }

    pub fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> bool {
        return self.environment.add_variable(identifier, r#type);
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<String, String> {
        return self.environment.lookup_variable(identifier); 
    }
}

