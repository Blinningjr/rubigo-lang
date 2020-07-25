use super::{
    Parser,
    TokenType,
    Expression,
};


/*
 * Defines all unary operators.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum UnOperator {
   Not,
   Minus,
}


/**
 * Defines binary operation in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct UnOp {
    pub un_op: UnOperator,
    pub expression: Expression,
}


/*
 * Defines all binary operators.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum BinOperator {
   Plus,
   Minus,
   Divition,
   Multiplication,
   Modilus,
   LessThen,
   GreaterThen,
   NotEqual,
   Equal,
   GreaterEqual,
   LessEqual,
   And,
   Or,
}


/**
 * Defines binary operation in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    pub bin_op: BinOperator,
    pub left_expression: Expression,
    pub right_expression: Expression,
}


impl Parser {
    /**
     * Parse unary operation.
     */
    pub(super) fn parse_un_op(&mut self) -> Expression {
        let un_op: UnOperator = self.parse_op_un();
        let expression: Expression = self.parse_expression();

        self.empty_tokens();
        return Expression::UnOp(Box::new(UnOp {
            un_op: un_op,
            expression: expression,
        }));
    }


    /**
     * Parse binary operation.
     */
    pub(super) fn parse_bin_op(&mut self, left_expression: Expression) -> Expression {
        let bin_op: BinOperator = self.parse_op_bin();
        let right_expression: Expression = self.parse_expression();
    
        self.empty_tokens();
        return Expression::BinOp(Box::new(BinOp {
            bin_op: bin_op,
            left_expression: left_expression,
            right_expression: right_expression,
        }));
    }


    /**
     * parse unary operator.
     */
    fn parse_op_un(&mut self) -> UnOperator {
        return match self.next_token().get_type() {
            TokenType::Not => UnOperator::Not,
            TokenType::Minus => UnOperator::Minus,
            _ => panic!("Expected unary operator"),
        };
    }
 

    /**
     * Parse binary operator.
     */
    fn parse_op_bin(&mut self) -> BinOperator {
        return match self.next_token().get_type() {
            TokenType::Plus => BinOperator::Plus,
            TokenType::Minus => BinOperator::Minus,
            TokenType::ForwardSlash => BinOperator::Divition,
            TokenType::Star => BinOperator::Multiplication,
            TokenType::Modilus => BinOperator::Modilus,
            TokenType::LessThen => BinOperator::LessThen,
            TokenType::GreaterThen => BinOperator::GreaterThen,
            TokenType::NotEqual => BinOperator::NotEqual,
            TokenType::Equal => BinOperator::Equal,
            TokenType::GreaterEqual => BinOperator::GreaterEqual,
            TokenType::LessEqual => BinOperator::LessEqual,
            TokenType::And => BinOperator::And,
            TokenType::Or => BinOperator::Or,
            _ => panic!("Expected bin operator"),
        };
    }


    /**
     * Checks if next token is a unary operator.
     */
    pub(super) fn is_un_op(&mut self) -> bool {
        return match self.peak().get_type() {
            TokenType::Not => true,
            TokenType::Minus => true,
            _ => false,
        };
    }
 

    /**
     * Checks if next token is a binary operator.
     */
    pub(super) fn is_bin_op(&mut self) -> bool {
        return match self.peak().get_type() {
            TokenType::Plus => true,
            TokenType::Minus => true,
            TokenType::ForwardSlash => true,
            TokenType::Star => true,
            TokenType::Modilus => true,
            TokenType::LessThen => true,
            TokenType::GreaterThen => true,
            TokenType::NotEqual => true,
            TokenType::Equal => true,
            TokenType::GreaterEqual => true,
            TokenType::LessEqual => true,
            TokenType::And => true,
            TokenType::Or => true,
            _ => false,
        };
    }
}

