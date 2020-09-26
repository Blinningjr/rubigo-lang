#![allow(dead_code)]

pub use super::{
    Interpreter,
    Span,
};

use crate::parser::literal::Literal;

pub use super::expressions::{
    Expression,
    FunctionCall,
    Variable,
};

impl Interpreter {
    pub(super) fn interpret_expression(&mut self, expression: Expression) -> Literal {
        return match expression {
            Expression::BinOp(binop) => self.interpret_binop(*binop),
            Expression::UnOp(unop) => self.interpret_unop(*unop),
            Expression::FunctionCall(function_call) => self.interpret_function_call(*function_call),
            Expression::Variable(variable) => self.interpret_variable(variable),
            Expression::Literal(literal) => literal,
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
        };
    }

    pub(super) fn interpret_function_call(&mut self, function_call: FunctionCall) -> Literal {
        return Literal::I32(Span::new(1, 0, 0))
        // TODO
    }

    fn interpret_variable(&mut self, variable: Variable) -> Literal {
        return Literal::I32(Span::new(1, 0, 0))
        // TODO
    }
}

