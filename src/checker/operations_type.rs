#![allow(dead_code)]


pub use super::{
    Checker,
    Span,
    ErrorLevel,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,

};

pub use super::r#type::{
    Type,
    MyTypes,
};


impl Checker {

    pub(super) fn get_binop_type(&mut self, binop: BinOp, original: Span<String>) -> Type {
        let left_expr: Type = self.get_expression_type(binop.left_expression.clone(), original.clone());
        let right_expr: Type = self.get_expression_type(binop.right_expression.clone(), original.clone()); 

        let mut r#type: Type = Type::new(MyTypes::Any);
        match self.check_binop_type(left_expr.r#type.clone(), binop.bin_op.get_fragment(), right_expr.r#type.clone()) {
            Some(t) => r#type = t,
            None => {
                let (line, offset): (usize, usize) = self.get_expression_location(binop.left_expression.clone());
                self.create_type_error(ErrorLevel::Error,
                                       format!("Binary operation {} {:#?} {} is not supported", left_expr.to_string(), binop.bin_op, right_expr.to_string()),
                                       original,
                                       line,
                                       offset);
            },
        };

        return r#type;
    }


    pub(super) fn get_unop_type(&mut self, unop: UnOp, original: Span<String>) -> Type {
        let expr: Type = self.get_expression_type(unop.expression.clone(), original.clone());

        let mut r#type: Type = Type::new(MyTypes::Any);
        match self.check_unop_type(unop.un_op.get_fragment(), expr.r#type.clone()) {
            Some(t) => r#type = t,
            None => {
                let (line, offset): (usize, usize) = self.get_expression_location(unop.expression.clone());
                self.create_type_error(ErrorLevel::Error,
                                       format!("Unary operation {:#?} {} is not supported", unop.un_op, expr.to_string()),
                                       original,
                                       line,
                                       offset);
            },
        };

        return r#type;
    }


    fn check_binop_type(& self, left: MyTypes, binop: BinOperator, right: MyTypes) -> Option<Type> {
        return match (left, binop, right) {
            (MyTypes::I32, BinOperator::Plus, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (MyTypes::F32, BinOperator::Plus, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            (MyTypes::I32, BinOperator::Minus, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (MyTypes::F32, BinOperator::Minus, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            (MyTypes::I32, BinOperator::Divition, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (MyTypes::F32, BinOperator::Divition, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            (MyTypes::I32, BinOperator::Multiplication, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (MyTypes::F32, BinOperator::Multiplication, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            (MyTypes::I32, BinOperator::Modilus, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (MyTypes::F32, BinOperator::Modilus, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            (MyTypes::I32, BinOperator::LessThen, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::LessThen, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::I32, BinOperator::GreaterThen, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::GreaterThen, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::I32, BinOperator::NotEqual, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::NotEqual, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::Bool, BinOperator::NotEqual, MyTypes::Bool) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::I32, BinOperator::Equal, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::Equal, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::Bool, BinOperator::Equal, MyTypes::Bool) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::I32, BinOperator::GreaterEqual, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::GreaterEqual, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::I32, BinOperator::LessEqual, MyTypes::I32) => Some(Type::new(MyTypes::Bool)),
            (MyTypes::F32, BinOperator::LessEqual, MyTypes::F32) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::Bool, BinOperator::And, MyTypes::Bool) => Some(Type::new(MyTypes::Bool)),
            
            (MyTypes::Bool, BinOperator::Or, MyTypes::Bool) => Some(Type::new(MyTypes::Bool)),

            _ => None,
        };
    }
    
    fn check_unop_type(& self, unop: UnOperator, expr: MyTypes) -> Option<Type> {
        return match (unop, expr) {
            (UnOperator::Not, MyTypes::Bool) => Some(Type::new(MyTypes::Bool)),

            (UnOperator::Minus, MyTypes::I32) => Some(Type::new(MyTypes::I32)),
            (UnOperator::Minus, MyTypes::F32) => Some(Type::new(MyTypes::F32)),
            
            _ => None,
        };
    }
}

