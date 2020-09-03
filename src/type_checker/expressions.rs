#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Expression,
    Literal,
};


impl TypeChecker {
    pub(super) fn get_expression_type(&mut self, expression: Expression) -> String {
        return match expression {
            Expression::Literal(_literal) => "i32".to_string(), //self.get_literal_type(literal),
            _ => panic!(),
        };
    }
}

