#![allow(dead_code)]

pub use super::{
    Span,
    environment::Environment,
    statement::Function,
};

pub use super::r#type::{
    Type,
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

    pub fn lookup_variable(&mut self, identifier: String, start_id: usize) -> Result<Type, String> {
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

    pub fn check_if_all_bodies_return(&mut self) -> bool {
        // TODO: Have a type for no type.
        if self.function.return_type.r#type.get_fragment() == "" {
           return true; 
        } else {
            return self.check_if_all_returns(0);
        }
    }

    pub fn check_if_all_returns(& self, env_id: usize) -> bool {
        if self.environments[env_id].returns_value {
            return true;
        } 

        let (_if_children, non_if_children): (Vec<usize>, Vec<usize>) = self.separate_if_env(self.find_childrens_ids(env_id));
        
        if non_if_children.len() == 0 {
            return false;
        } else {
            for id in non_if_children {
                if self.check_if_all_returns(id) {
                    return true;
                }
            } 
            return false; 
        }
    }

    fn find_childrens_ids(& self, parent_id: usize) -> Vec<usize> {
        let mut childrens_ids: Vec<usize> = vec!();
        for env in self.environments.iter() {
            match env.previus_id {
                Some(id) => {
                    if id == parent_id {
                        childrens_ids.push(env.id); 
                    }
                },
                None => (),
            };
        }
        return childrens_ids;
    }

    /*
     * Returns (if_envs, non_if_envs)
     */
    fn separate_if_env(& self, envs: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        let mut if_children: Vec<usize> = vec!();
        let mut non_if_children: Vec<usize> = vec!();
        for id in envs {
            if self.environments[id].if_body {
                if_children.push(id);
            } else {
                non_if_children.push(id);
            }
        }
        return (if_children, non_if_children);
    }
}

