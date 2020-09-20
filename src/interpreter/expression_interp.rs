#![allow(dead_code)]

pub use super::{
    Interpreter,
    Span,
};

pub use super::expressions::{
    Expression,
    FunctionCall,
    Variable,
};

impl Interpreter {
    pub(super) fn interpret_expression(&mut self, expression: Expression, original: Span<String>) -> () {
        return match expression {
            Expression::BinOp(binop) => self.interpret_binop(*binop, original),
            Expression::UnOp(unop) => self.interpret_unop(*unop, original),
            Expression::FunctionCall(function_call) => self.interpret_function_call(*function_call, original),
            Expression::Variable(variable) => self.interpret_variable(variable, original),
            Expression::Literal(literal) => (),
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
        };
    }

    fn interpret_function_call(&mut self, function_call: FunctionCall, original: Span<String>) -> () {
    }

    fn interpret_variable(&mut self, variable: Variable, original: Span<String>) -> () {
    }
}

