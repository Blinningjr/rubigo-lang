#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Literal,
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

    fn get_function_call_type(&mut self, function_call: FunctionCall) -> String {
        let mut inputs_type: Vec<String> = vec!();
        for expr in function_call.parameters.iter() {
            inputs_type.push(self.get_expression_type(expr.clone()));
        }

        let function: Function = self.lookup_function(function_call.identifier);
        let mut parameters_type: Vec<String> = vec!();
        for (_, type_dec) in function.parameters {
            parameters_type.push(type_dec.r#type.get_fragment()); 
        }

        if inputs_type.len() != parameters_type.len() {
            self.create_error("type error: not the same amount of parameters".to_string());
        } else {
            for i in 0..inputs_type.len() {
                if inputs_type[i] != parameters_type[i] {
                    self.create_error(format!("type error: parameter {} wrong type", i));
                } 
            }
        }

        return function.return_type.r#type.get_fragment();
    }

    fn get_variable_type(&mut self, variable: Variable) -> String {
        return self.lookup_variable(variable.identifier);
    }
}

