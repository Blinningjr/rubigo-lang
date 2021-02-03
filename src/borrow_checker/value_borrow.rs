#![allow(dead_code)]


pub use super::{
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowValue {
    Literal(bool), // (Mutable)
    Pointer(bool, Option<usize>, usize), // (Mutable, Function, Env)
}

