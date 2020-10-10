#![allow(dead_code)]

pub use super::{
    Checker,
    literal::Literal,
    Span,
    ErrorLevel,
};

pub use super::r#type::{
    Type,
};

pub use super::expressions::{
    Expression,
    Variable,
};


impl Checker {
    pub(super) fn get_expression_type(&mut self, expression: Expression, original: Span<String>) -> Type {
        return match expression {
            Expression::BinOp(binop) => self.get_binop_type(*binop, original),
            Expression::UnOp(unop) => self.get_unop_type(*unop, original),
//            Expression::FunctionCall(function_call) => self.get_function_call_type(*function_call, original),
            Expression::Variable(variable) => self.get_variable_type(variable, original),
            Expression::Literal(literal) => self.get_literal_type(literal),
            Expression::Borrow(expr) => {
                let mut expr_type: Type = self.get_expression_type(*expr, original);
                if expr_type.borrow {
                    panic!("error");
                    // TODO: Change it to type error instead of panic
                }
                expr_type.borrow = true;
                return expr_type;
            },
            Expression::Mutable(expr) => {
                let mut expr_type: Type = self.get_expression_type(*expr, original);
                if expr_type.mutable {
                    panic!("error");
                    // TODO: Change it to type error instead of panic
                }
                expr_type.mutable = true;
                return expr_type;
            },
            Expression::DeRefrence(expr) => {
                let mut expr_type: Type = self.get_expression_type(*expr, original);
                if !expr_type.borrow {
                    panic!("error");
                    // TODO: Change it to type error instead of panic
                }
                expr_type.borrow = false;
                return expr_type;
            },
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
            _ => panic!("TODO: Needs to be implemented"),
        };
    }


    fn get_variable_type(&mut self, var: Variable, original: Span<String>) -> Type {
        match self.module.get_variable(var.identifier.get_fragment(), 
                                            self.current_func,
                                            self.current_env) {
            Some(var) => return var.get_type(),
            None => panic!("TODO: add type error here"),
        };
    }
}


