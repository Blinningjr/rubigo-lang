#![allow(dead_code)]

pub use super::{
    Interpreter,
    Literal,
    Span,
    Value,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,
};

impl Interpreter {
    pub(super) fn interpret_binop(&mut self, binop: BinOp) -> Value {
        let left: Value = self.interpret_expression(binop.left_expression);
        let right: Value = self.interpret_expression(binop.right_expression);
        return match binop.bin_op.get_fragment() {
            BinOperator::Plus => bin_add(left, right),
            BinOperator::Minus => bin_sub(left,right),
            BinOperator::Divition => bin_div(left, right),
            BinOperator::Multiplication => bin_mul(left, right),
            BinOperator::Modilus => bin_mod(left, right),
            BinOperator::LessThen => bin_lt(left, right),
            BinOperator::GreaterThen => bin_gt(left, right),
            BinOperator::NotEqual => bin_neq(left, right),
            BinOperator::Equal => bin_eq(left, right),
            BinOperator::GreaterEqual => bin_geq(left, right),
            BinOperator::LessEqual => bin_leq(left, right),
            BinOperator::And => bin_and(left, right),
            BinOperator::Or => bin_or(left, right),
            BinOperator::Dummy => panic!("Fatal Interpreter Error"),
        };
    }

    pub(super) fn interpret_unop(&mut self, unop: UnOp) -> Value {
        let expression: Value = self.interpret_expression(unop.expression);
        return match unop.un_op.get_fragment() {
            UnOperator::Not => un_not(expression),
            UnOperator::Minus => un_minus(expression),
            _ => panic!("Not Implemented"),        
        };
    }
}

fn bin_neq(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval != rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval != rval),
        (Value::Bool(lval), Value::Bool(rval)) => Value::Bool(lval != rval),
        (Value::Char(lval), Value::Char(rval)) => Value::Bool(lval != rval),
        (Value::String(lval), Value::String(rval)) => Value::Bool(lval != rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_eq(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval == rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval == rval),
        (Value::Bool(lval), Value::Bool(rval)) => Value::Bool(lval == rval),
        (Value::Char(lval), Value::Char(rval)) => Value::Bool(lval == rval),
        (Value::String(lval), Value::String(rval)) => Value::Bool(lval == rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_add(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::I32(lval + rval),
        (Value::F32(lval), Value::F32(rval)) => Value::F32(lval + rval),
        (Value::String(lval), Value::String(rval)) => Value::String(lval + &rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_sub(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::I32(lval - rval),
        (Value::F32(lval), Value::F32(rval)) => Value::F32(lval - rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_div(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::I32(lval / rval),
        (Value::F32(lval), Value::F32(rval)) => Value::F32(lval / rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_mul(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::I32(lval * rval),
        (Value::F32(lval), Value::F32(rval)) => Value::F32(lval * rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_mod(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::I32(lval % rval),
        (Value::F32(lval), Value::F32(rval)) => Value::F32(lval % rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_lt(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval < rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval < rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_gt(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval > rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval > rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_leq(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval <= rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval <= rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}
    
fn bin_geq(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::I32(lval), Value::I32(rval)) => Value::Bool(lval >= rval),
        (Value::F32(lval), Value::F32(rval)) => Value::Bool(lval >= rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_and(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::Bool(lval), Value::Bool(rval)) => Value::Bool(lval && rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn bin_or(left: Value, right: Value) -> Value {
    return match (left, right) {
        (Value::Bool(lval), Value::Bool(rval)) => Value::Bool(lval || rval),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn un_not(expr: Value) -> Value {
    return match expr {
        Value::Bool(val) => Value::Bool(!val),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

fn un_minus(expr: Value) -> Value {
    return match expr {
        Value::I32(val) => Value::I32(-val),
        Value::F32(val) => Value::F32(-val),
        _ => panic!("Fatal Interpreter Error"),        
    };
}

