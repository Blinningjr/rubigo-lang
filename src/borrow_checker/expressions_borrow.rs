#![allow(dead_code)]

pub use super::{
    BorrowChecker,
    literal::Literal,
    Span,
    ErrorLevel,
    environment_borrow::BorrowFunction,
    operations::UnOperator,
};

pub use super::value_borrow::BorrowValue;


pub use super::expressions::{
    Expression,
    Variable,
    FunctionCall,
};


impl BorrowChecker {

    pub(super) fn check_expression(&mut self, expression: Expression) -> BorrowValue {
        match expression {
            Expression::BinOp(binop) => self.check_binop(*binop),
            Expression::UnOp(unop) => self.check_unop(*unop),
            Expression::FunctionCall(func_call) => self.check_function_call(*func_call),
            Expression::Variable(var) => self.check_variable(var),
            Expression::Literal(literal) => self.check_litteral(literal),
            Expression::Borrow(expr) => self.check_borrow(*expr),
            Expression::DeRefrence(expr) => self.check_deref(*expr),
            Expression::Mutable(expr) => self.check_mutable(*expr),
            Expression::Dummy => panic!("Parser failed! Dummy expression in borrow checker."),
        }
    }

    fn check_function_call(&mut self, func_call: FunctionCall) -> BorrowValue {
        for expr in func_call.parameters.iter() {
            self.check_expression(expr.clone());
        }
        return BorrowValue::Literal(false);
    }

    fn check_variable(&mut self, variable: Variable) -> BorrowValue {
        // TODO: look up value and return.
        // Note: if pointer then the borrowing stack rules should be checked.
        return BorrowValue::Literal(false);
    }
    
    fn check_litteral(&mut self, literal: Literal) -> BorrowValue {
        return BorrowValue::Literal(false);
    }

    fn check_borrow(&mut self, expression: Expression) -> BorrowValue {
        // TODO: create pointer
        return BorrowValue::Literal(false);
    }
    
    fn check_deref(&mut self, expression: Expression) -> BorrowValue {
        // TODO: derefrence pointer
        return BorrowValue::Literal(false);
    }
    
    fn check_mutable(&mut self, expression: Expression) -> BorrowValue {
        // TODO: set the value to mutable
        return BorrowValue::Literal(false);
    }
    
    pub(super) fn get_expression_location(& self, expression: Expression) -> (usize, usize) {
        match expression {
            Expression::BinOp(bin_op) => {
                return self.get_expression_location(bin_op.left_expression);
            },
            Expression::UnOp(un_op) => {
                let op: Span<UnOperator> = un_op.un_op;
                return (op.get_line(), op.get_offset());
            },
            Expression::FunctionCall(func_call) => {
                let ident: Span<String> = func_call.identifier; 
                return (ident.get_line(), ident.get_offset());
            },
            Expression::Variable(var) => {
                let ident: Span<String> = var.identifier; 
                return (ident.get_line(), ident.get_offset());
            },
            Expression::Literal(literal) => {
                return match literal {
                    Literal::I32(span) => (span.get_line(), span.get_offset()),
                    Literal::F32(span) => (span.get_line(), span.get_offset()),
                    Literal::Bool(span) => (span.get_line(), span.get_offset()),
                    Literal::Char(span) => (span.get_line(), span.get_offset()),
                    Literal::String(span) => (span.get_line(), span.get_offset()),
                    Literal::Dummy => panic!("Fatal Error!!!"),
                };
            },
            Expression::Borrow(expr) => {
                return self.get_expression_location(*expr);
            },
            Expression::DeRefrence(expr) => {
                return self.get_expression_location(*expr);
            },
            Expression::Mutable(expr) => {
                return self.get_expression_location(*expr);
            },
            Expression::Dummy => panic!("Fatal Error!!!"),
        };
    }
}


