#![allow(dead_code)]


pub use super::{
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowValue {
    Literal(bool), // (Mutable)
    Pointer(bool, usize, usize, usize), // (Mutable, Function stack pointer, Env stack pointer, stack pointer, borrow stack pointer)
    UnknownPointer, // Pointer that is unknown.
}

