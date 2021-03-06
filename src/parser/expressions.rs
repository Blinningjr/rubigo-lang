use super::{
    Parser,
    Token,
    TokenType,
    Literal,
    UnOp,
    BinOp,
    ErrorLevel,
    Span,
};


/**
 * Defines all expressions in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinOp(Box<BinOp>),
    UnOp(Box<UnOp>),
    FunctionCall(Box<FunctionCall>),
    Variable(Variable),
    Literal(Literal),
    Borrow(Box<Expression>),
    DeRefrence(Box<Expression>),
    Mutable(Box<Expression>),
    Dummy,
}


/**
 * Defines function call in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub id: usize,
    pub original: Span<String>,
    pub identifier: Span<String>,
    pub parameters: Vec<Expression>,
}


/**
 * Defines variable in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub identifier: Span<String>,
}


impl Parser {
    /**
     * Parse expression.
     */
    pub(super) fn parse_expression(&mut self) -> Expression {
        let expression: Expression;

        if self.is_tokentype(TokenType::Identifier) {
            expression = self.parse_identifier_expression();

        } else if self.is_un_op() {
            expression = self.parse_un_op(); 

        } else if self.is_literal() {
            expression = Expression::Literal(self.parse_literal());
        
        } else if self.is_tokentype(TokenType::ParenthesisStart) {
            let _start: Token = self.next_token();
            expression = self.parse_expression();

            let _end: Token = self.parse_type(TokenType::ParenthesisEnd);

        } else if self.is_tokentype(TokenType::Borrow) {
            let _borrow: Token = self.next_token();
            expression = Expression::Borrow(Box::new(self.parse_singel_expression()));

        } else if self.is_tokentype(TokenType::Star) {
            let _star: Token = self.next_token();
            expression = Expression::DeRefrence(Box::new(self.parse_singel_expression()));

        } else if self.is_tokentype(TokenType::Mut) {
            let _mut: Token = self.next_token();
            expression = Expression::Mutable(Box::new(self.parse_singel_expression()));

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Expression".to_string());

            return Expression::Dummy;
        }
        
        if self.is_bin_op() {
            return self.parse_bin_op(expression); 
        } else {
            return expression;
        }
    }

    fn parse_singel_expression(&mut self) -> Expression {  
        if self.is_tokentype(TokenType::Identifier) {
            return self.parse_identifier_expression();

        } else if self.is_un_op() {
            return self.parse_un_op(); 

        } else if self.is_literal() {
            return Expression::Literal(self.parse_literal());
        
        } else if self.is_tokentype(TokenType::ParenthesisStart) {
            let _start: Token = self.next_token();
            let expression: Expression = self.parse_expression();
            let _end: Token = self.parse_type(TokenType::ParenthesisEnd);
            
            return expression;

        } else if self.is_tokentype(TokenType::Borrow) {
            let _borrow: Token = self.next_token();
            return Expression::Borrow(Box::new(self.parse_singel_expression()));

        } else if self.is_tokentype(TokenType::Star) {
            let _star: Token = self.next_token();
            return Expression::DeRefrence(Box::new(self.parse_singel_expression()));

        } else if self.is_tokentype(TokenType::Mut) {
            let _mut: Token = self.next_token();
            return Expression::Mutable(Box::new(self.parse_singel_expression()));

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Expression".to_string());

            return Expression::Dummy;
        }
    }


    /**
     * Parse expressions starting with identifier.
     */
    fn parse_identifier_expression(&mut self) -> Expression {
        let  original_start: usize = self.get_original_start();
        let identifier: Token = self.next_token();
        if self.is_tokentype(TokenType::ParenthesisStart) {
            return self.parse_function_call(identifier, original_start); 
        
        } else {
            return Expression::Variable(Variable {
                identifier: self.create_span(identifier.get_value(), & identifier),
            });
        }
    }


    /**
     * Parse function call.
     * :param identifier: Token of type identifier.
     */
    pub(super) fn parse_function_call(&mut self, identifier: Token,
                                      original_start: usize) -> Expression {
        let _param_start: Token = self.next_token();

        let mut parameters: Vec<Expression> = Vec::new();
        let mut until: bool = true;
        while until {
            if self.is_tokentype(TokenType::ParenthesisEnd) { 
                let _end: Token = self.next_token();
                until = false;

            } else {
                parameters.push(self.parse_expression());
                if self.is_tokentype(TokenType::Comma) {
                    let _comma: Token = self.next_token();

                } else if self.is_tokentype(TokenType::ParenthesisEnd) {
                    let _end: Token = self.next_token(); 
                    until = false;

                } else {
                    self.create_error(ErrorLevel::Error, "Expected ')'".to_string());
                    until = false;
                }
            }
        }

        return Expression::FunctionCall(Box::new(FunctionCall {
            id: self.body_id,
            original: self.get_original(original_start),
            identifier: self.create_span(identifier.get_value(), & identifier),
            parameters: parameters,
        }));
    }
}

