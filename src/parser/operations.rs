use super::{
    Parser,
    TokenType,
    Token,
    Expression,
    ErrorLevel,
    Error,
    SyntaxError,
};


/*
 * Defines all unary operators.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum UnOperator {
   Not,
   Minus,
   Dummy,
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
   Dummy,
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
        match self.peak().get_type() {
            TokenType::Not => {
                let _token: Token = self.next_token();
                return UnOperator::Not;
            },
            TokenType::Minus => {
                let _token: Token = self.next_token();
                return UnOperator::Minus
            },
            _ => {
                let original_start: usize = self.get_original_start();

                let token: Token = self.peak();
                let code: String = self.get_original(original_start);
                
                self.error_handler.add(Error::SyntaxError(SyntaxError {
                    level: ErrorLevel::Error,
                    message: "Expected unary operator".to_string(),
                    code: code,
                    line: token.get_line(),
                    offset: token.get_offset(),
                }));
                return UnOperator::Dummy;
            }
        };
    }
 

    /**
     * Parse binary operator.
     */
    fn parse_op_bin(&mut self) -> BinOperator {
        match self.peak().get_type() {
            TokenType::Plus => {
                let _token: Token = self.next_token();
                return BinOperator::Plus;
            },
            TokenType::Minus => {
                let _token: Token = self.next_token();
                return BinOperator::Minus;
            },
            TokenType::ForwardSlash => {
                let _token: Token = self.next_token();
                return BinOperator::Divition;
            },
            TokenType::Star => {
                let _token: Token = self.next_token();
                return BinOperator::Multiplication;
            },
            TokenType::Modilus => {
                let _token: Token = self.next_token();
                return BinOperator::Modilus;
            },
            TokenType::LessThen => {
                let _token: Token = self.next_token();
                return BinOperator::LessThen;
            },
            TokenType::GreaterThen => {
                let _token: Token = self.next_token();
                return BinOperator::GreaterThen;
            },
            TokenType::NotEqual => {
                let _token: Token = self.next_token();
                return BinOperator::NotEqual;
            },
            TokenType::Equal => {
                let _token: Token = self.next_token();
                return BinOperator::Equal;
            },
            TokenType::GreaterEqual => {
                let _token: Token = self.next_token();
                return BinOperator::GreaterEqual;
            },
            TokenType::LessEqual => {
                let _token: Token = self.next_token();
                return BinOperator::LessEqual;
            },
            TokenType::And => {
                let _token: Token = self.next_token();
                return BinOperator::And;
            },
            TokenType::Or => {
                let _token: Token = self.next_token();
                return BinOperator::Or;
            },
            _ => { 
                let original_start: usize = self.get_original_start() - 1;
                
                let token: Token = self.peak();
                let code: String = self.get_original(original_start);
                
                self.error_handler.add(Error::SyntaxError(SyntaxError {
                    level: ErrorLevel::Error,
                    message: "Expected binary operator".to_string(),
                    code: code,
                    line: token.get_line(),
                    offset: token.get_offset(),
                }));
                return BinOperator::Dummy;     
            },
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

