#![allow(dead_code)]


pub use super::{
    Checker,
    Span,
    ErrorLevel,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,

};

pub use super::r#type::{
    Type,
};


impl Checker {

    pub(super) fn get_binop_type(&mut self, binop: BinOp, original: Span<String>) -> Type {
        let left_expression_type: Type = self.get_expression_type(binop.left_expression.clone(), original.clone());
        let right_expression_type: Type = self.get_expression_type(binop.right_expression.clone(), original.clone()); 

        // TODO: Check that it is a legal operation.

        return left_expression_type;
    }


    pub(super) fn get_unop_type(&mut self, unop: UnOp, original: Span<String>) -> Type {
        let expression_type: Type = self.get_expression_type(unop.expression.clone(), original.clone());

        // TODO: Check that it is a legal operation.

        return expression_type;
    }

}

