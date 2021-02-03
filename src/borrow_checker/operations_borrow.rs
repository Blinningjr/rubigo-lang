#![allow(dead_code)]


pub use super::{
    BorrowChecker,
    Span,
    ErrorLevel,
};

pub use super::value_borrow::BorrowValue;

pub use super::operations::{
    BinOp,
    UnOp,
};



impl BorrowChecker {
    pub(super) fn check_binop(&mut self, binop: BinOp) -> BorrowValue {
        self.check_expression(binop.left_expression.clone());
        self.check_expression(binop.right_expression.clone());
        return BorrowValue::Literal(false);
    }
    
    pub(super) fn check_unop(&mut self, unop: UnOp) -> BorrowValue {
        self.check_expression(unop.expression.clone());
        return BorrowValue::Literal(false);
    }

}

