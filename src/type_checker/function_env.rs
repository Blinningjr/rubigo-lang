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

    pub fn add_function(&mut self, identifier: Span<String>, env_id: usize, body_id: usize) -> bool {
        return self.environments[body_id].add_function(identifier, env_id);
    }

    pub fn lookup_function_id(&mut self, identifier: String, start_id: usize) -> Result<usize, String> {
        if self.function.identifier.get_fragment() == identifier {
            return Ok(self.id);
        }

        let mut env_id_r: Option<usize> = Some(start_id); 
        loop {
            match env_id_r {
                Some(env_id) => {
                    match self.environments[env_id].lookup_function(identifier.clone()) {
                        Ok(id) => {
                            return Ok(id);
                        },
                        Err(_) => env_id_r = self.environments[env_id].previus_id,
                    };   
                },
                None => return Err("function not decleared".to_string()),
            };
        }
    }

    pub fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>, body_id: usize) -> bool {
        return self.environments[body_id].add_variable(identifier, r#type);
    }

    pub fn lookup_variable(&mut self, identifier: String, start_id: usize) -> Result<String, String> {
        let mut env_id_r: Option<usize> = Some(start_id); 
        loop {
            match env_id_r {
                Some(env_id) => {
                    match self.environments[env_id].lookup_variable(identifier.clone()) {
                        Ok(var) => {
                            return Ok(var);
                        },
                        Err(_) => env_id_r = self.environments[env_id].previus_id,
                    };   
                },
                None => return Err("variable not decleared".to_string()),
            };
        }
    }

    pub fn create_env(&mut self, current_id: usize) -> usize {
        let new_id: usize = self.environments.len();
        self.environments.push(Environment::new(new_id, Some(current_id)));
        return new_id;
    }
}

