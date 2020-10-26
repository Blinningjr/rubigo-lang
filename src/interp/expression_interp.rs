#![allow(dead_code)]

pub use super::{
    Interpreter,
    Span,
    statement::Function,
    Literal,
    Value,
};

pub use super::expressions::{
    Expression,
    FunctionCall,
    Variable,
};


impl Interpreter {
    pub(super) fn interpret_expression(&mut self, expression: Expression) -> Value {
        return match expression {
            Expression::BinOp(binop) => self.interpret_binop(*binop),
            Expression::UnOp(unop) => self.interpret_unop(*unop),
//            Expression::FunctionCall(function_call) => self.interpret_function_call(*function_call),
//            Expression::Variable(variable) => self.interpret_variable(variable),
            Expression::Literal(literal) => self.interpret_literal(literal),
//            Expression::Borrow(expr) => self.interpret_expression(*expr), //TODO
//            Expression::DeRefrence(expr) => self.interpret_expression(*expr), //TODO
//            Expression::Mutable(expr) => self.interpret_expression(*expr), //TODO
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
            _ => panic!("Parser failed! Dummy expression in type checker."),
        };
    }


    fn interpret_literal(&mut self, literal: Literal) -> Value {
        return match literal {
            Literal::I32(span) => Value::I32(span.get_fragment()),
            Literal::F32(span) => Value::F32(span.get_fragment()),
            Literal::Bool(span) => Value::Bool(span.get_fragment()),
            Literal::Char(span) => Value::Char(span.get_fragment()),
            Literal::String(span) => Value::String(span.get_fragment()),
            Literal::Dummy => panic!("Fatal Interpreter Error"),
        };
    }
}

