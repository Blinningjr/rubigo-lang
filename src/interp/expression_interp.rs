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
            Expression::FunctionCall(function_call) => self.interpret_function_call(*function_call).unwrap(),
            Expression::Variable(variable) => self.interpret_variable(variable),
            Expression::Literal(literal) => self.interpret_literal(literal),
            Expression::Borrow(expr) => self.interpret_borrow(*expr),
            Expression::DeRefrence(expr) => self.interpret_derefrence(*expr),
            Expression::Mutable(expr) => self.interpret_expression(*expr),
            Expression::Dummy => panic!("Parser failed! Dummy expression in type checker."),
            //_ => panic!("Fatal Interpreter Error"),
        };
    }

    pub fn interpret_function_call(&mut self, func_call: FunctionCall) -> Option<Value> {
        if func_call.identifier.get_fragment() == "print" {
            let val: Value = self.interpret_expression(func_call.parameters[0].clone());
            println!("{}", val.string());
            return None;
        } 
        
        let function: Function = self.get_function(func_call.identifier.get_fragment());
        
        let mut values: Vec<Value> = vec!();
        for expr in func_call.parameters {
            values.push(self.interpret_expression(expr));
        }

        return self.interpret_function(function, values);
    }

    fn interpret_variable(&mut self, var: Variable) -> Value {
        return self.get_variable(var.identifier.get_fragment());
    }

    fn interpret_borrow(&mut self, expression: Expression) -> Value {
        let ident: String = match expression {
            Expression::Variable(var) => var.identifier.get_fragment(),
            Expression::Mutable(expr) => match *expr {
                Expression::Variable(var) => var.identifier.get_fragment(),
                _ => panic!("Fatal Interpreter Error"),
            },
            _ => panic!("Fatal Interpreter Error"),
        };
        
        return self.get_pointer(ident);
    }

    fn interpret_derefrence(&mut self, expression: Expression) -> Value {
        let pointer = match self.interpret_expression(expression) {
            Value::Pointer(p) => p,
            _ => panic!("Fatal Interpreter Error"),
        };
        return self.get_value(pointer);
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

