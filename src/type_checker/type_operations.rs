#![allow(dead_code)]


pub use super::{
    TypeChecker,
    operations,
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
    compare_types,
};


impl TypeChecker {

    pub(super) fn get_binop_type(&mut self, binop: BinOp, original: Span<String>) -> Type {
        let left_expression_type: Type = self.get_expression_type(binop.left_expression.clone(), original.clone());
        let right_expression_type: Type = self.get_expression_type(binop.right_expression, original.clone()); 
        let binop_type: (Type, Type) = self.binop_type(binop.bin_op.get_fragment());

        if !compare_types(&left_expression_type, &right_expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(binop.left_expression);
            self.create_type_error(ErrorLevel::Error, "type error binop".to_string(), original, line, offset);

        } else if !compare_types(&binop_type.1, &left_expression_type.clone()) {
            let (line, offset): (usize, usize) = self.get_expression_location(binop.left_expression);
            self.create_type_error(ErrorLevel::Error, "type error binop".to_string(), original, line, offset);  
        }

        if binop_type.0 != Type::Any || binop_type.0 != Type::Number {
            return binop_type.0;

        } else {
            return left_expression_type;
        }
    }

    /*
     *  Return Operator type and type of expression expected.
     */
    fn binop_type(&mut self, binop: BinOperator) -> (Type, Type) {
        return match binop {
            BinOperator::Plus => (Type::Number, Type::Number),
            BinOperator::Minus => (Type::Number, Type::Number),
            BinOperator::Divition => (Type::Number, Type::Number),
            BinOperator::Multiplication => (Type::Number, Type::Number),
            BinOperator::Modilus => (Type::Number, Type::Number),
            BinOperator::LessThen => (Type::Number, Type::Number),
            BinOperator::GreaterThen => (Type::Number, Type::Number),
            BinOperator::NotEqual => (Type::Any, Type::Any),
            BinOperator::Equal => (Type::Any, Type::Any),
            BinOperator::GreaterEqual => (Type::Number, Type::Number),
            BinOperator::LessEqual => (Type::Number, Type::Number),
            BinOperator::And => (Type::Custom("bool".to_string()), Type::Custom("bool".to_string())),
            BinOperator::Or => (Type::Custom("bool".to_string()), Type::Custom("bool".to_string())),
            BinOperator::Dummy => panic!("Parser failed! Dummy BinOperator in type checker"),
        };
    } 

    pub(super) fn get_unop_type(&mut self, unop: UnOp, original: Span<String>) -> Type {
        let expression_type: Type = self.get_expression_type(unop.expression.clone(), original.clone());
        let unop_type: Type = self.unop_type(unop.un_op.get_fragment());

        if !compare_types(&unop_type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(unop.expression);
            self.create_type_error(ErrorLevel::Error, "type error unop".to_string(), original, line, offset); 
        }
        return expression_type;
    }

    fn unop_type(&mut self, unop: UnOperator) -> Type {
        return match unop {
            UnOperator::Not => Type::Custom("bool".to_string()),
            UnOperator::Minus => Type::Number,
            UnOperator::Dummy => panic!("Parser failed! Dummy UnOperator in type checker"),
        };
    } 
}

