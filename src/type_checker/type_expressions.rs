#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Literal,
};

pub use super::expressions::{
    Expression,
    FunctionCall,
    Variable,
};

pub use super::operations::{
    BinOp,
    UnOp,
};


impl TypeChecker {
    pub(super) fn get_expression_type(&mut self, expression: Expression) -> String {
        return match expression {
            Expression::BinOp(binop) => self.get_binop_type(*binop),
            Expression::UnOp(unop) => self.get_unop_type(*unop),
            Expression::FunctionCall(function_call) => self.get_function_call_type(*function_call),
            Expression::Variable(variable) => self.get_variable_type(variable),
            Expression::Literal(literal) => self.get_literal_type(literal),
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
        };
    }

    // TODO: Also Check the type of the parameters.
    fn get_function_call_type(&mut self, function_call: FunctionCall) -> String {
        return self.lookup_function(function_call.identifier);
    }

    fn get_variable_type(&mut self, variable: Variable) -> String {
        return self.lookup_variable(variable.identifier);
    }
}

