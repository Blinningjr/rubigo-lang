use super::{
    Parser,
    TokenType,
    Token,
    Expression,
    ErrorLevel,
    Span,
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
    pub un_op: Span<UnOperator>,
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
    pub bin_op: Span<BinOperator>,
    pub left_expression: Expression,
    pub right_expression: Expression,
}


impl Parser {
    /**
     * Parse unary operation.
     */
    pub(super) fn parse_un_op(&mut self) -> Expression {
        let un_op: Span<UnOperator> = self.parse_op_un();
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
        let bin_op: Span<BinOperator> = self.parse_op_bin();
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
    fn parse_op_un(&mut self) -> Span<UnOperator> {
        match self.peak().get_type() {
            TokenType::Not => {
                let token: Token = self.next_token();
                return self.create_span(UnOperator::Not, & token);
            },
            TokenType::Minus => {
                let token: Token = self.next_token();
                return self.create_span(UnOperator::Minus, & token);
            },
            _ => {
                self.create_error(ErrorLevel::Error, "Expected a unary operator.".to_string());
                return Span::new(UnOperator::Dummy, 0, 0);
            }
        };
    }
 

    /**
     * Parse binary operator.
     */
    fn parse_op_bin(&mut self) -> Span<BinOperator> {
        match self.peak().get_type() {
            TokenType::Plus => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Plus, & token);
            },
            TokenType::Minus => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Minus, & token);
            },
            TokenType::ForwardSlash => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Divition, & token);
            },
            TokenType::Star => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Multiplication, & token);
            },
            TokenType::Modilus => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Modilus, & token);
            },
            TokenType::LessThen => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::LessThen, & token);
            },
            TokenType::GreaterThen => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::GreaterThen, & token);
            },
            TokenType::NotEqual => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::NotEqual, & token);
            },
            TokenType::Equal => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Equal, & token);
            },
            TokenType::GreaterEqual => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::GreaterEqual, & token);
            },
            TokenType::LessEqual => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::LessEqual, & token);
            },
            TokenType::And => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::And, & token);
            },
            TokenType::Or => {
                let token: Token = self.next_token();
                return self.create_span(BinOperator::Or, & token);
            },
            _ => { 
                self.create_error(ErrorLevel::Error, "Expected a binary operator".to_string());
                return Span::new(BinOperator::Dummy, 0, 0);     
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

