#![allow(dead_code)]

pub use super::{
    FunctionEnv,
    Environment,
    Statement,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Modual {
    pub environments: Vec<FunctionEnv>,
    pub environment: Environment,

    pub current_env_id: Option<usize>,
    pub current_body_id: usize,

    pub ast: Statement,
}

impl Modual {
    pub fn new(ast: Statement) -> Modual {
        return Modual{
            environments: vec!(),
            environment: Environment::new(0, None),
            
            current_env_id: None,
            current_body_id: 0,
            
            ast: ast,
        };
    }
}

