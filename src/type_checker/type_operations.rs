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
        let right_expression_type: Type = self.get_expression_type(binop.right_expression.clone(), original.clone()); 
        let binop_type: (Type, Type) = self.binop_type(binop.bin_op.get_fragment());

        if !compare_types(&left_expression_type, &right_expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(binop.left_expression);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Both the left and right expression of a binary operation must be of the same type. left: {} right {}",
                                           left_expression_type.to_string(),
                                           right_expression_type.to_string()),
                                   original,
                                   line,
                                   offset);

        } else if !compare_types(&binop_type.1, &left_expression_type.clone()) {
            let (line, offset): (usize, usize) = self.get_expression_location(binop.left_expression.clone());
            self.create_type_error(ErrorLevel::Error,
                                   format!("Binary operator {:?} requiers that the expressions are of type {}",
                                           binop.bin_op.get_fragment(),
                                           binop_type.1.to_string()),
                                   original,
                                   line,
                                   offset);  
        }

        return binop_type.0;
    }

    /*
     *  Return Operator type and type of expression expected.
     */
    fn binop_type(&mut self, binop: BinOperator) -> (Type, Type) {
        return match binop {
            BinOperator::Plus => (Type::Number(false, false), Type::Number(false, false)),
            BinOperator::Minus => (Type::Number(false, false), Type::Number(false, false)),
            BinOperator::Divition => (Type::Number(false, false), Type::Number(false, false)),
            BinOperator::Multiplication => (Type::Number(false, false), Type::Number(false, false)),
            BinOperator::Modilus => (Type::Number(false, false), Type::Number(false, false)),
            BinOperator::LessThen => (Type::Custom("bool".to_string(), false, false), Type::Number(false, false)),
            BinOperator::GreaterThen => (Type::Custom("bool".to_string(), false, false), Type::Number(false, false)),
            BinOperator::NotEqual => (Type::Custom("bool".to_string(), false, false), Type::Any),
            BinOperator::Equal => (Type::Custom("bool".to_string(), false, false), Type::Any),
            BinOperator::GreaterEqual => (Type::Custom("bool".to_string(), false, false), Type::Number(false, false)),
            BinOperator::LessEqual => (Type::Custom("bool".to_string(), false, false), Type::Number(false, false)),
            BinOperator::And => (Type::Custom("bool".to_string(), false, false), Type::Custom("bool".to_string(), false, false)),
            BinOperator::Or => (Type::Custom("bool".to_string(), false, false), Type::Custom("bool".to_string(), false, false)),
            BinOperator::Dummy => panic!("Parser failed! Dummy BinOperator in type checker"),
        };
    } 

    pub(super) fn get_unop_type(&mut self, unop: UnOp, original: Span<String>) -> Type {
        let expression_type: Type = self.get_expression_type(unop.expression.clone(), original.clone());
        let unop_type: Type = self.unop_type(unop.un_op.get_fragment());

        if !compare_types(&unop_type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(unop.expression);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Expected type {} got {}", unop_type.to_string(), expression_type.to_string()),
                                   original,
                                   line, 
                                   offset); 
        }
        return unop_type;
    }

    fn unop_type(&mut self, unop: UnOperator) -> Type {
        return match unop {
            UnOperator::Not => Type::Custom("bool".to_string(), false, false),
            UnOperator::Minus => Type::Number(false, false),
            UnOperator::Dummy => panic!("Parser failed! Dummy UnOperator in type checker"),
        };
    } 
}

