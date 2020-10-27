#![allow(dead_code)]


pub use super::{
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub ident: Option<String>,

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
    Any,
}


impl Type {


    pub fn new(mytype: MyTypes) -> Type {
        return Type{
            ident: None,

            borrow: false,
            mutable:false,
            r#type: mytype,
        };
    }

    pub fn parse(r#type: &str, borrow: bool, mutable: bool) -> Option<Type> {
        return match r#type {
            "i32" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::I32,
            }),
            "f32" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::F32,
            }),
            "bool" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::Bool,
            }),
            "char" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::Char,
            }),
            "String" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::String,
            }),
            " ANY" => Some(Type{
                ident: None,
                borrow: borrow,
                mutable: mutable,
                r#type: MyTypes::Any,
            }),
            _ => None,
        };
    }

    pub fn get_literal_type(literal: Literal) -> Type {
        return Type{
            ident: None,

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
        match (&self.r#type, &other.r#type) {
            (_, MyTypes::Any) => return true,
            (MyTypes::Any, _) => return true,
            _ => (),
        };
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

    pub fn to_string(& self) -> String {
            let mut s = "".to_string();
            if self.borrow { 
                s = format!("{}&", s);
            }

            if self.mutable {
                    s = format!("{}mut", s);
            }

            let t = match self.r#type {
                MyTypes::I32 => "i32".to_string(),
                MyTypes::F32 => "f32".to_string(),
                MyTypes::Bool => "bool".to_string(),
                MyTypes::Char => "char".to_string(),
                MyTypes::String => "String".to_string(),
                _ => panic!("fatal error"), 
            };
            return format!("{} {}", s, t);
    }
}

