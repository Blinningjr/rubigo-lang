#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Literal,
    Span,
    ErrorLevel,
};

pub use super::r#type::{
    Type,
    compare_types,
};

pub use super::statement::{
    Function,
};

pub use super::expressions::{
    Expression,
    FunctionCall,
    Variable,
};

pub use super::operations::{
    BinOp,
    UnOp,
    UnOperator,
};


impl TypeChecker {
    pub(super) fn get_expression_type(&mut self, expression: Expression, original: Span<String>) -> Type {
        return match expression {
            Expression::BinOp(binop) => self.get_binop_type(*binop, original),
            Expression::UnOp(unop) => self.get_unop_type(*unop, original),
            Expression::FunctionCall(function_call) => self.get_function_call_type(*function_call, original),
            Expression::Variable(variable) => self.get_variable_type(variable, original),
            Expression::Literal(literal) => self.get_literal_type(literal),
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
        };
    }

    fn get_function_call_type(&mut self, function_call: FunctionCall, original: Span<String>) -> Type {
        let mut inputs_type: Vec<Type> = vec!();
        let mut inputs_location: Vec<(usize, usize)> = vec!();
        for expr in function_call.parameters.iter() {
            inputs_type.push(self.get_expression_type(expr.clone(), original.clone()));
            inputs_location.push(self.get_expression_location(expr.clone()));
        }

        let function: Function;
        match self.lookup_function(function_call.identifier.clone()) {
            Ok(func) => function = func,
            Err(msg) => {
               self.create_type_error(ErrorLevel::Error, msg, original, function_call.identifier.get_line(), function_call.identifier.get_offset());
               return Type::Any;
            },
        };
        let mut parameters_type: Vec<Type> = vec!();
        for (_, type_dec) in function.parameters {
            parameters_type.push(Type::Custom(type_dec.r#type.get_fragment())); 
        }

        if inputs_type.len() != parameters_type.len() {
            self.create_type_error(ErrorLevel::Error,
                                   format!("Function {:#?} requiers {} parameters, recived {}", function.identifier.get_fragment(), parameters_type.len(), inputs_type.len()),
                                   original,
                                   function_call.identifier.get_line(),
                                   function_call.identifier.get_offset());
        } else {
            for i in 0..inputs_type.len() {
                if !compare_types(&inputs_type[i], &parameters_type[i]) {
                    self.create_type_error(ErrorLevel::Error, format!("type error parameter {} wrong type", i), original.clone(), inputs_location[i].0, inputs_location[i].1);
                } 
            }
        }

        return Type::Custom(function.return_type.r#type.get_fragment());
    }

    fn get_variable_type(&mut self, variable: Variable, original: Span<String>) -> Type {
        return self.lookup_variable(variable.identifier, original);
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
            Expression::Dummy => panic!("Fatal Error!!!"),
        };
    }
}

