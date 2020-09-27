#![allow(dead_code)]

pub use super::{
    Interpreter,
    Span,
    Function,
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
        let function: Function = self.get_function(function_call.identifier.get_fragment(), function_call.id);
       
        if function.identifier.get_fragment() == "print" {
            let text: Literal = self.interpret_expression(function_call.parameters[0].clone());
            println!("{}", lit_to_string(text));
            return Literal::Dummy;
        } 


        let mut values: Vec<Literal> = vec!();
        for expr in function_call.parameters {
            values.push(self.interpret_expression(expr));
        }

        return self.interpret_function(function, values);
    }

    fn interpret_variable(&mut self, variable: Variable) -> Literal {
        return self.get_variable(variable.identifier.get_fragment());
    }
}


fn lit_to_string(literal: Literal) -> String {
    return match literal {
        Literal::I32(val) => format!("{}", val.get_fragment()),
        Literal::F32(val) => format!("{}", val.get_fragment()),
        Literal::Bool(val) => format!("{}", val.get_fragment()),
        Literal::Char(val) => format!("{}", val.get_fragment()),
        Literal::String(val) => format!("{}", val.get_fragment()),
        _ => panic!("Fatal interpreter error"),
    };
}

