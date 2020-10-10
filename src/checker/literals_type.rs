#![allow(dead_code)]


pub use super::{
    Checker,
    literal::Literal,

};

pub use super::r#type::Type;


impl Checker {
    pub(super) fn get_literal_type(&mut self, literal: Literal) -> Type {
        return Type{
            borrow: false,
            mutable: false,
            r#type: literal,
        };
    }
}

