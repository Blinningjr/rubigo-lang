use super::{
    Parser,
    Token,
    TokenType,
    Literal,
    UnOp,
    BinOp,
};


/**
 * Defines all expressions in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinOp(Box<BinOp>),
    UnOp(Box<UnOp>),
    FunctionCall(Box<FunctionCall>),
    Literal(Literal),
}


/**
 * Defines function call in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub identifier: String,
    pub parameters: Vec<Expression>,
}


/**
 * Defines variable in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub identifier: String,
}


impl Parser {
    /**
     * Parse expression.
     */
    pub(super) fn parse_expression(&mut self) -> Expression {
        let mut expression: Expression;

        if self.is_tokentype(TokenType::Ident) {
            expression = self.parse_identifier_expression();

        } else if self.is_un_op() {
            expression = self.parse_un_op(); 

        } else if self.is_literal() {
            expression = Expression::Literal(self.parse_literal());
        
        } else if self.is_tokentype(TokenType::ParenthesisStart) {
            let _start: Token = self.next_token(true);
            expression = self.parse_expression();
            let _end: Token = self.parse_type(TokenType::ParenthesisEnd);

        } else {
            panic!("Expression not Implemented yet.");
        }
        
        if self.is_bin_op() {
            return self.parse_bin_op(expression); 
        } else {
            return expression;
        }
    }


    /**
     * Parse expressions starting with identifier.
     */
    fn parse_identifier_expression(&mut self) -> Expression {
        let identifier: Token = self.next_token(true);
        if self.is_tokentype(TokenType::ParenthesisStart) {
            self.empty_tokens();
            return self.parse_function_call(identifier); 
        
        } else {
            let variable: Expression = Expression::Literal(Literal::String(identifier.get_value()));

            self.empty_tokens();
            return variable; 
        }
    }


    /**
     * Parse function call.
     * :param identifier: Token of type identifier.
     */
    pub(super) fn parse_function_call(&mut self, identifier: Token) -> Expression {
        let _param_start: Token = self.next_token(true);

        let mut parameters: Vec<Expression> = Vec::new();
        let mut until: bool = true;
        while until {
            if self.is_tokentype(TokenType::ParenthesisEnd) { 
                let _end: Token = self.next_token(true);
                until = false;

            } else {
                parameters.push(self.parse_expression());
                if self.is_tokentype(TokenType::Comma) {
                    let _comma: Token = self.next_token(true);

                } else if self.is_tokentype(TokenType::ParenthesisEnd) {
                    let _end: Token = self.next_token(true); 
                    until = false;

                } else {
                    panic!("Error Function call");
                }
            }
        }

        self.empty_tokens();
        return Expression::FunctionCall(Box::new(FunctionCall {
            identifier: identifier.get_value(),
            parameters: parameters,
        }));
    }
}

