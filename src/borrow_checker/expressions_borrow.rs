#![allow(dead_code)]

pub use super::{
    BorrowChecker,
    literal::Literal,
    Span,
    ErrorLevel,
    operations::UnOperator,
    BorrowEnvironments,
};

pub use super::value_borrow::BorrowValue;


pub use super::expressions::{
    Expression,
    Variable,
    FunctionCall,
};


impl BorrowChecker {

    pub(super) fn check_expression(&mut self, envs: &mut BorrowEnvironments, expression: Expression, original: &Span<String>) -> BorrowValue {
        match expression {
            Expression::BinOp(binop) => self.check_binop(envs, *binop, original),
            Expression::UnOp(unop) => self.check_unop(envs, *unop, original),
            Expression::FunctionCall(func_call) => self.check_function_call(envs, *func_call, original),
            Expression::Variable(var) => self.check_variable(envs, var, original),
            Expression::Literal(literal) => self.check_litteral(envs, literal, original),
            Expression::Borrow(expr) => self.check_borrow(envs, *expr, original),
            Expression::DeRefrence(expr) => self.check_deref(envs, *expr, original),
            Expression::Mutable(expr) => self.check_mutable(envs, *expr, original),
            Expression::Dummy => panic!("Parser failed! Dummy expression in borrow checker."),
        }
    }

    pub fn check_function_call(&mut self, envs: &mut BorrowEnvironments, func_call: FunctionCall, original: &Span<String>) -> BorrowValue {
        let mut pointers = vec!();
        for expr in func_call.parameters.iter() {
            match self.check_expression(envs, expr.clone(), original) {
                BorrowValue::Pointer(a, b, c, d) => {
                    for p in pointers.iter() {
                        match p {
                            BorrowValue::Pointer(_a1, b1, c1, _d1) => {
                                if b == *b1 && c == *c1 {
                                    let (line, offset) = self.get_expression_location(expr.clone());
                                    let msg = "pointer arguments need to be distinct".to_string();
                                    self.create_borrow_error(ErrorLevel::Error, msg, original.clone(), line, offset);
                                } 
                            },
                            _ => panic!("Fatal error"),
                        };
                    }
                    pointers.push(BorrowValue::Pointer(a, b, c, d));
                },
                _ => (),
            };
        }
        return BorrowValue::Literal(false);
    }

    fn check_variable(&mut self, envs: &mut BorrowEnvironments, variable: Variable, original: &Span<String>) -> BorrowValue {
        let (val, err) = envs.get_value(variable.identifier.get_fragment()).unwrap();
        if let Some(msg) = err {
            self.create_borrow_error(ErrorLevel::Error,
                                     msg,
                                     original.clone(),
                                     variable.identifier.get_line(),
                                     variable.identifier.get_offset());
        }
        return val;
    }
    
    fn check_litteral(&mut self, _envs: &mut BorrowEnvironments, _literal: Literal, _original: &Span<String>) -> BorrowValue {
        return BorrowValue::Literal(false);
    }

    fn check_borrow(&mut self, envs: &mut BorrowEnvironments, expression: Expression, _original: &Span<String>) -> BorrowValue {
        match expression {
            Expression::Variable(var) => {
                return envs.create_pointer(var.identifier.get_fragment(), false);
            },
            Expression::Mutable(expr) => {
                match *expr {
                    Expression::Variable(var) => {
                        return envs.create_pointer(var.identifier.get_fragment(), true);
                    },
                    _ => panic!("Fatal type checker error"),
                };                
            },
            _ => panic!("Fatal type checker error"),
        };
    }
    
    fn check_deref(&mut self, envs: &mut BorrowEnvironments, expression: Expression, original: &Span<String>) -> BorrowValue {
        let val = self.check_expression(envs, expression.clone(), original);
        match val {
            BorrowValue::Literal(mutable) => {
                let (line, offset) = self.get_expression_location(expression);
                self.create_borrow_error(ErrorLevel::Error,
                                         "Can't dereference literal".to_string(),
                                         original.clone(),
                                         line,
                                         offset);
                return BorrowValue::Literal(mutable);
            },
            BorrowValue::Pointer(_, env, stack, b_stack) => {
                let (val, err) =  envs.envs[env].stack[stack].get_value(b_stack);
                if let Some(msg) = err {
                    let (line, offset) = self.get_expression_location(expression);
                    self.create_borrow_error(ErrorLevel::Error,
                                             msg,
                                             original.clone(),
                                             line,
                                             offset);
                }
                return val;
            },
            BorrowValue::UnknownPointer =>  return BorrowValue::UnknownPointer, //unimplemented!(),
        };
    }
    
    fn check_mutable(&mut self, envs: &mut BorrowEnvironments, expression: Expression, original: &Span<String>) -> BorrowValue {
        let val = self.check_expression(envs, expression, original);
        return match val {
            BorrowValue::Literal(_) => BorrowValue::Literal(true),
            BorrowValue::Pointer(_, env, stack, b_stack) => BorrowValue::Pointer(true, env, stack, b_stack),
            BorrowValue::UnknownPointer => panic!("Fatal type checker error"),
        };
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


