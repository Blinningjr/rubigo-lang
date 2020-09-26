#![allow(dead_code)]

pub use super::{
    Interpreter,
    Literal,
    Span,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,
};


impl Interpreter {

    pub(super) fn interpret_binop(&mut self, binop: BinOp) -> Literal {
        let left: Literal = self.interpret_expression(binop.left_expression);
        let right: Literal = self.interpret_expression(binop.right_expression);
        return match binop.bin_op.get_fragment() {
            BinOperator::Plus => bin_add(left, right),
            BinOperator::Minus => bin_sub(left,right),
//            BinOperator::Divition,
//            BinOperator::Multiplication,
//            BinOperator::Modilus,
//            BinOperator::LessThen,
//            BinOperator::GreaterThen,
//            BinOperator::NotEqual,
//            BinOperator::Equal,
//            BinOperator::GreaterEqual,
//            BinOperator::LessEqual,
            BinOperator::And => {
                let left_b: bool = self.get_bool(left);
                let right_b: bool = self.get_bool(right);
                return Literal::Bool(Span::new(left_b && right_b, 0, 0));
            },
            BinOperator::Or => Literal::Bool(Span::new(self.get_bool(left) || self.get_bool(right), 0, 0)),
            BinOperator::Dummy => panic!("Fatal Error"),
            _ => panic!("Not Implemented"),        
        };
    }

    pub(super) fn interpret_unop(&mut self, unop: UnOp) -> Literal {
        let expression: Literal = self.interpret_expression(unop.expression);
        return match unop.un_op.get_fragment() {
            UnOperator::Not => Literal::Bool(Span::new(!self.get_bool(expression), 0, 0)),
            UnOperator::Minus => un_minus(expression),
            _ => panic!("Not Implemented"),        
        };
    }

}

fn bin_add(left: Literal, right: Literal) -> Literal {
    match left {
        Literal::I32(lval) => {
            return match right {
                Literal::I32(rval) => Literal::I32(Span::new(lval.get_fragment() + rval.get_fragment(), lval.get_line(), lval.get_offset())),
                _ => panic!("Interpreter type error"),        
            };
        },
        Literal::F32(lval) => {
            return match right {
                Literal::F32(rval) => Literal::F32(Span::new(lval.get_fragment() + rval.get_fragment(), lval.get_line(), lval.get_offset())),
                _ => panic!("Interpreter type error"),        
            };
        },
        _ => panic!("Interpreter type error"),        
    };
}

fn bin_sub(left: Literal, right: Literal) -> Literal {
    match left {
        Literal::I32(lval) => {
            return match right {
                Literal::I32(rval) => Literal::I32(Span::new(lval.get_fragment() - rval.get_fragment(), lval.get_line(), lval.get_offset())),
                _ => panic!("Interpreter type error"),        
            };
        },
        Literal::F32(lval) => {
            return match right {
                Literal::F32(rval) => Literal::F32(Span::new(lval.get_fragment() - rval.get_fragment(), lval.get_line(), lval.get_offset())),
                _ => panic!("Interpreter type error"),        
            };
        },
        _ => panic!("Interpreter type error"),        
    };
}

fn un_minus(expr: Literal) -> Literal {
    match expr {
        Literal::I32(val) => return Literal::I32(Span::new(val.get_fragment() * (-1), val.get_line(), val.get_offset())),
        Literal::F32(val) => return Literal::F32(Span::new(val.get_fragment() * (-1.0), val.get_line(), val.get_offset())),
        _ => panic!("Interpreter type error"),        
    };
}

