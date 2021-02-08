#![allow(dead_code)]


pub use super::{
    BorrowChecker,
    Span,
    ErrorLevel,
    BorrowEnvironments,
};

pub use super::value_borrow::BorrowValue;

pub use super::operations::{
    BinOp,
    UnOp,
};



impl BorrowChecker {
    pub(super) fn check_binop(&mut self, envs: &mut BorrowEnvironments, binop: BinOp, original: &Span<String>) -> BorrowValue {
        self.check_expression(envs, binop.left_expression.clone(), original);
        self.check_expression(envs, binop.right_expression.clone(), original);
        return BorrowValue::Literal(false);
    }
    
    pub(super) fn check_unop(&mut self, envs: &mut BorrowEnvironments, unop: UnOp, original: &Span<String>) -> BorrowValue {
        self.check_expression(envs, unop.expression.clone(), original);
        return BorrowValue::Literal(false);
    }

}

