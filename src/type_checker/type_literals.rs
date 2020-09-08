#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Literal,
};

pub use super::r#type::Type;


impl TypeChecker {
    pub(super) fn get_literal_type(&mut self, literal: Literal) -> Type {
        return match literal {
            Literal::I32(_) => Type::Custom("i32".to_string()),
            Literal::F32(_) => Type::Custom("f32".to_string()),
            Literal::Bool(_) => Type::Custom("bool".to_string()),
            Literal::Char(_) => Type::Custom("char".to_string()),
            Literal::String(_) => Type::Custom("String".to_string()),
            Literal::Dummy => panic!("Parser fail! Dummy literal in type checker."),
        };
    }
}

