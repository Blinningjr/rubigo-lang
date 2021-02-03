#![allow(dead_code)]

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
}

