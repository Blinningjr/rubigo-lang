#![allow(dead_code)]

pub use super::{
    Checker,
    literal::Literal,
    Span,
    ErrorLevel,
    environment_type::TypeFunction,
};

pub use super::r#type::{
    Type,
};

pub use super::expressions::{
    Expression,
    Variable,
    FunctionCall,
};


impl Checker {
    pub(super) fn get_expression_type(&mut self, expression: Expression, original: Span<String>) -> Type {
        return match expression {
            Expression::BinOp(binop) => self.get_binop_type(*binop, original),
            Expression::UnOp(unop) => self.get_unop_type(*unop, original),
            Expression::FunctionCall(function_call) => {
                match self.get_function_call_type(*function_call, original) {
                    Some(t) => return t,
                    None => panic!("TODO: Add type error"),
                };
            },
            Expression::Variable(variable) => self.get_variable_type(variable, original),
            Expression::Literal(literal) => Type::get_literal_type(literal),
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
        let (_, _, t) = self.get_variable(var.identifier.get_fragment(), original);
        return t.get_type();
    }

    fn get_function_call_type(&mut self, func_call: FunctionCall, original: Span<String>) -> Option<Type> {
        let mut input_type: Vec<Type> = vec!();
        for expr in func_call.parameters.iter() {
            input_type.push(self.get_expression_type(expr.clone(), original.clone()));
        }

        let func: TypeFunction = self.get_function(func_call.identifier.get_fragment());

        if input_type.len() != func.parameters.len() {
            panic!("TODO: Add type error");
        } else {
            for i in 0..input_type.len() {
                if !input_type[i].same_type(& func.parameters[i].1) {
                    panic!("TODO: Add type error");
                }
            }
        }

        return func.return_type;
    }
}


