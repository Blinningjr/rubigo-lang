#![allow(dead_code)]

pub use super::environment_type::{
    TypeEnvironments,
    TypeFunction,
    TypeVariable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeModule {
    pub mod_envs: TypeEnvironments,
    pub mod_funcs: Vec<TypeFunction>,
}

impl TypeModule {
    pub fn new() -> TypeModule {
        return TypeModule{
            mod_envs: TypeEnvironments::new(),
            mod_funcs: vec!(),
        };
    }

    pub fn get_variable(& self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> Option<(Option<usize>, usize, &TypeVariable)> {
       match func_id {
            Some(id) => {
                match self.mod_funcs[id].get_variable(ident.clone(), env_id) {
                   Some((env_id, var)) => return Some((func_id, env_id, var)),
                   None => (),
                };
            },
            None => (),
       }; 

        return match self.mod_envs.get_variable(ident, mod_env_id) {
            Some((env_id, var)) => Some((None, env_id, var)),
            None => None,
        };
    }
    
    pub fn get_function_id(& self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> Option<usize> {
       match func_id {
            Some(id) => {
                match self.mod_funcs[id].get_function_id(ident.clone(), env_id) {
                   Some(var) => return Some(var),
                   None => (),
                };
            },
            None => (),
       }; 

        return self.mod_envs.get_function_id(ident, mod_env_id);
    }

    pub fn add_borrow_as_mut(&mut self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> Result<bool, TypeVariable> {
        match self.get_variable(ident.clone(), func_id, env_id, mod_env_id) {
            Some((f_id, e_id, _)) => {
                match f_id {
                    Some(func) => {
                        return self.mod_funcs[func].environments.envs[e_id].add_borrow_as_mut(&ident);
                    },
                    None => {
                        return self.mod_envs.envs[e_id].add_borrow_as_mut(&ident);
                    },
                };
            },
            None => return Ok(false),
        };
    }


    pub fn add_borrow(&mut self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> Result<bool, TypeVariable> {
        match self.get_variable(ident.clone(), func_id, env_id, mod_env_id) {
            Some((f_id, e_id, _)) => {
                match f_id {
                    Some(func) => {
                        return self.mod_funcs[func].environments.envs[e_id].add_borrow(&ident);
                    },
                    None => {
                        return self.mod_envs.envs[e_id].add_borrow(&ident);
                    },
                };
            },
            None => return Ok(false),
        };
    }


    pub fn remove_borrow_as_mut(&mut self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> () {
        match self.get_variable(ident.clone(), func_id, env_id, mod_env_id) {
            Some((f_id, e_id, _)) => {
                match f_id {
                    Some(func) => {
                        self.mod_funcs[func].environments.envs[e_id].remove_borrow_as_mut(&ident);
                    },
                    None => {
                        self.mod_envs.envs[e_id].remove_borrow_as_mut(&ident);
                    },
                };
            },
            None => panic!("Fatal error"),
        };
    }

    pub fn remove_borrow(&mut self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> () {
        match self.get_variable(ident.clone(), func_id, env_id, mod_env_id) {
            Some((f_id, e_id, _)) => {
                match f_id {
                    Some(func) => {
                        self.mod_funcs[func].environments.envs[e_id].remove_borrow(&ident);
                    },
                    None => {
                        self.mod_envs.envs[e_id].remove_borrow(&ident);
                    },
                };
            },
            None => panic!("Fatal error"),
        };
    }
}

