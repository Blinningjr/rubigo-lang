#![allow(dead_code)]

pub use super::value_borrow::BorrowValue;

pub use super::environment_borrow::{
    BorrowEnvironments,
    BorrowFunction,
    BorrowVariable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BorrowModule {
    pub mod_envs: BorrowEnvironments,
    pub mod_funcs: Vec<BorrowFunction>, 
}

impl BorrowModule {
    pub fn new() -> BorrowModule {
        return BorrowModule{
            mod_envs: BorrowEnvironments::new(),
            mod_funcs: vec!(),
        };
    }
    
    pub fn get_variable(& self, ident: String, func_id: Option<usize>, env_id: usize, mod_env_id: usize) -> Option<(Option<usize>, usize, &BorrowVariable)> {
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
}

