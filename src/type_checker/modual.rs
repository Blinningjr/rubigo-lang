#![allow(dead_code)]

pub use super::{
    FunctionEnv,
    Environment,
    ModualBody,
    Span,
    Variable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Modual {
    pub environments: Vec<FunctionEnv>,
    pub mod_envs: Vec<Environment>,

    pub current_env_id: Option<usize>,
    pub current_body_id: usize,

    pub mod_body_id: usize,

    pub ast: ModualBody,
}

impl Modual {
    pub fn new(ast: ModualBody) -> Modual {
        return Modual{
            environments: vec!(),
            mod_envs: vec!(Environment::new(0, None)),
            
            current_env_id: None,
            current_body_id: 0,
            mod_body_id: 0,
            
            ast: ast,
        };
    }


   
    pub fn lookup_variable_in_mod_envs(&mut self, identifier: Span<String>) -> Result<Variable, String> { 
        let mut env_body_id_r: Option<usize> = Some(self.mod_body_id);  
        loop {
            match env_body_id_r {
                Some(env_id) =>{
                    match self.mod_envs[env_id].lookup_variable(identifier.get_fragment()) {
                        Ok(val) => return Ok(val),
                        Err(_) => env_body_id_r = self.mod_envs[env_id].previus_id,
                    }
                },
                None => {
                    return Err(format!("Variable {:#?} not in scope.", identifier));
                },
            };
        
        } 
    }
}

