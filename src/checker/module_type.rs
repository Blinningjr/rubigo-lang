#![allow(dead_code)]

pub use super::environment_type::{
    TypeEnvironments,
    TypeFunction,
    TypeVarMem,
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

    pub fn get_variable(& self, ident: String, func_id: Option<usize>, env_id: usize) -> Option<&TypeVarMem> {
       match func_id {
            Some(id) => {
                match self.mod_funcs[id].get_variable(ident.clone(), env_id) {
                   Some(var) => return Some(var),
                   None => (),
                };
            },
            None => (),
       }; 

        return self.mod_envs.get_variable(ident, env_id);
    }
    
    pub fn get_function_id(& self, ident: String, func_id: Option<usize>, env_id: usize) -> Option<usize> {
       match func_id {
            Some(id) => {
                match self.mod_funcs[id].get_function_id(ident.clone(), env_id) {
                   Some(var) => return Some(var),
                   None => (),
                };
            },
            None => (),
       }; 

        return self.mod_envs.get_function_id(ident, env_id);
    }

}

