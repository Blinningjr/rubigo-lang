#![allow(dead_code)]

pub use super::{
    Interpreter,
    Span,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,

};


impl Interpreter {

    pub(super) fn interpret_binop(&mut self, binop: BinOp, original: Span<String>) -> () {
    }

    pub(super) fn interpret_unop(&mut self, unop: UnOp, original: Span<String>) -> () {
    }
}

