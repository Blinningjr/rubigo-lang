#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Literal,
};


impl TypeChecker {
    pub(super) fn get_literal_type(&mut self, literal: Literal) -> String {
        return match literal {
            Literal::I32(_) => "i32".to_string(),
            Literal::F32(_) => "f32".to_string(),
            Literal::Bool(_) => "bool".to_string(),
            Literal::Char(_) => "char".to_string(),
            Literal::String(_) => "String".to_string(),
            Literal::Dummy => panic!("Parser fail! Dummy literal in type checker."),
        };
    }
}

