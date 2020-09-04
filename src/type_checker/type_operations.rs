#![allow(dead_code)]


pub use super::{
    TypeChecker,
    operations,
};

pub use super::operations::{
    BinOp,
    BinOperator,
    UnOp,
    UnOperator,

};


impl TypeChecker {

    pub(super) fn get_binop_type(&mut self, binop: BinOp) -> String {
        let left_expression_type: String = self.get_expression_type(binop.left_expression);
        let right_expression_type: String = self.get_expression_type(binop.right_expression); 
        let binop_type: (String, String) = self.binop_type(binop.bin_op.get_fragment());

        if left_expression_type.clone() != right_expression_type {
            self.create_error("type error binop".to_string());

        } else if !self.check_type(binop_type.1, left_expression_type.clone()) {
            self.create_error("type error binop".to_string());  
        }

        if binop_type.0 != "ANY" || binop_type.0 != "NUMBER" {
            return binop_type.0;

        } else {
            return left_expression_type;
        }
    }

    /*
     *  Return Operator type and type of expression expected.
     */
    fn binop_type(&mut self, binop: BinOperator) -> (String, String) {
        return match binop {
            BinOperator::Plus => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::Minus => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::Divition => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::Multiplication => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::Modilus => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::LessThen => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::GreaterThen => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::NotEqual => ("ANY".to_string(), "ANY".to_string()),
            BinOperator::Equal => ("ANY".to_string(), "ANY".to_string()),
            BinOperator::GreaterEqual => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::LessEqual => ("NUMBER".to_string(), "NUMBER".to_string()),
            BinOperator::And => ("bool".to_string(), "bool".to_string()),
            BinOperator::Or => ("bool".to_string(), "bool".to_string()),
            BinOperator::Dummy => panic!("Parser failed! Dummy BinOperator in type checker"),
        };
    } 

    pub(super) fn get_unop_type(&mut self, unop: UnOp) -> String {
        let expression_type: String = self.get_expression_type(unop.expression);
        let unop_type: String = self.unop_type(unop.un_op.get_fragment());

        if !self.check_type(unop_type, expression_type.clone()) {
            self.create_error("type error unop".to_string()); 
        }
        return expression_type;
    }

    fn unop_type(&mut self, unop: UnOperator) -> String {
        return match unop {
            UnOperator::Not => "bool".to_string(),
            UnOperator::Minus => "NUMBER".to_string(),
            UnOperator::Dummy => panic!("Parser failed! Dummy UnOperator in type checker"),
        };
    } 

    fn check_type(&mut self, op_type: String, expression_type: String) -> bool {
        if op_type == "ANY" {
            return true;
        } else if op_type == "NUMBER" {
            if expression_type == "i32" || expression_type == "f32" {
                return true;
            }
        } else if op_type == expression_type {
            return true;
        }

        return false;
    }
}

