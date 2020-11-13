#![allow(dead_code)]

pub use super::{
    Checker,
    literal::Literal,
    Span,
    ErrorLevel,
    environment_type::TypeFunction,
    operations::UnOperator,
};

pub use super::r#type::{
    Type,
    MyTypes,
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
                match self.get_function_call_type(*function_call.clone(), original.clone()) {
                    Some(t) => return t,
                    None => {
                        self.create_type_error(ErrorLevel::Error,
                                               format!("Function call must return"),
                                               original,
                                               function_call.identifier.get_line(),
                                               function_call.identifier.get_offset());

                        return Type::new(MyTypes::Any, self.get_location());
                    },
                };
            },
            Expression::Variable(variable) => self.get_variable_type(variable, original),
            Expression::Literal(literal) => Type::get_literal_type(literal, self.get_location()),
            Expression::Borrow(expr) => {
                let mut expr_type: Type = self.get_expression_type(*expr.clone(), original.clone());
                if expr_type.borrow {
                    let (line, offset): (usize, usize) = self.get_expression_location(*expr.clone());
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Can't borrow already borrowed value"),
                                           original.clone(),
                                           line,
                                           offset);
                }
                expr_type.borrow = true;
               
                match expr_type.ident.clone() {
                    Some(ident) => {
                        if expr_type.mutable {
                            self.add_borrow_as_mut(ident, original.clone());
                        } else {
                            self.add_borrow(ident, original.clone());
                        }
                    }, 
                    None => (),
                };

                return expr_type;
            },
            Expression::Mutable(expr) => {
                // TODO: Check that the variable is mutable
                let mut expr_type: Type = self.get_expression_type(*expr.clone(), original.clone());
                if expr_type.mutable {
                    let (line, offset): (usize, usize) = self.get_expression_location(*expr.clone());
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Value is already mutable"),
                                           original.clone(),
                                           line,
                                           offset);
                }
                expr_type.mutable = true;
                return expr_type;
            },
            Expression::DeRefrence(expr) => {
                let mut expr_type: Type = self.get_expression_type(*expr.clone(), original.clone());
                if !expr_type.borrow {
                    let (line, offset): (usize, usize) = self.get_expression_location(*expr.clone());
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Can't derefrence a none borrowed value"),
                                           original.clone(),
                                           line,
                                           offset);
                }
                expr_type.borrow = false;
                return expr_type;
            },
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
        };
    }

    fn get_variable_type(&mut self, var: Variable, original: Span<String>) -> Type {
        let (_func, _env, mut t) = match self.get_variable(var.identifier.get_fragment(), original) {
            Some(val) => val,
            None => return Type::new(MyTypes::Any, self.get_location()), 
        };
        t.r#type.ident = Some(var.identifier.get_fragment()); 
        return t.r#type;
    }

    pub(super) fn get_function_call_type(&mut self, func_call: FunctionCall, original: Span<String>) -> Option<Type> {
        let mut input_type: Vec<Type> = vec!();
        for expr in func_call.parameters.iter() {
            input_type.push(self.get_expression_type(expr.clone(), original.clone()));
        }

        let func: TypeFunction = match self.get_function(func_call.identifier.get_fragment(), original.clone()) {
            Some(val) => val,
            None => return None,
        };

        if input_type.len() != func.parameters.len() {
            self.create_type_error(ErrorLevel::Error,
                                   format!("Expected {} paramters got {}", func.parameters.len(), input_type.len()),
                                   original.clone(),
                                   func_call.identifier.get_line(),
                                   func_call.identifier.get_offset());
        } else {
            for i in 0..input_type.len() {
                if !input_type[i].same_type(& func.parameters[i].1) {
                    let (line, offset): (usize, usize) = self.get_expression_location(func_call.parameters[i].clone());
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Expected type {} got {}, for parameter {}",
                                                   func.parameters[i].1.to_string(),
                                                   input_type[i].to_string(),
                                                   func.og_func.parameters[i].identifier.get_fragment()),
                                           original.clone(),
                                           line,
                                           offset);
                }
            }
        }

        return func.return_type;
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


