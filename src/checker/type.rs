#![allow(dead_code)]


pub use super::{
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub borrow: bool,
    pub mutable: bool,

    pub r#type: MyTypes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MyTypes {
    I32,
    F32,
    Bool,
    Char,
    String,
}


impl Type {

    pub fn new(mytype: MyTypes) -> Type {
        return Type{
            borrow: false,
            mutable:false,
            r#type: mytype,
        };
    }

    pub fn get_literal_type(literal: Literal) -> Type {
        return Type{
            borrow: false,
            mutable: false,

            r#type: match literal {
                Literal::I32(_) => MyTypes::I32,
                Literal::F32(_) => MyTypes::F32,
                Literal::Bool(_) => MyTypes::Bool,
                Literal::Char(_) => MyTypes::Char,
                Literal::String(_) => MyTypes::String,
                _ => panic!("fatal error"),
            },
        };
    }

    pub fn same_type(& self, other: &Type) -> bool {
        if self.borrow == other.borrow {
            if self.mutable == other.mutable {
                return match (&self.r#type, &other.r#type) {
                    (MyTypes::I32, MyTypes::I32) => true,
                    (MyTypes::F32, MyTypes::F32) => true,
                    (MyTypes::Bool, MyTypes::Bool) => true,
                    (MyTypes::Char, MyTypes::Char) => true,
                    (MyTypes::String, MyTypes::String) => true,
                    _ => false,
                };
            }
        }
        return false;
    }
}

