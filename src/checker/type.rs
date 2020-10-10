#![allow(dead_code)]


pub use super::{
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub borrow: bool,
    pub mutable: bool,

    pub r#type: Literal,
}


impl Type {
    pub fn same_type(& self, other: &Type) -> bool {
        if self.borrow == other.borrow {
            if self.mutable == other.mutable {
                return match (&self.r#type, &other.r#type) {
                    (Literal::I32(_), Literal::I32(_)) => true,
                    (Literal::F32(_), Literal::F32(_)) => true,
                    (Literal::Bool(_), Literal::Bool(_)) => true,
                    (Literal::Char(_), Literal::Char(_)) => true,
                    (Literal::String(_), Literal::String(_)) => true,
                    _ => false,
                };
            }
        }
        return false;
    }
}

