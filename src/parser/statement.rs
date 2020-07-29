use super::{
    Parser,
    Token,
    TokenType,
    TypeDecleration,
    Expression,
    ErrorLevel,
};


/**
 * Defines all expressions in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Function(Box<Function>),
    While(Box<While>),
    If(Box<If>),
    Let(Let),
    Assignment(Assignment),
    Return(Return),
    Body(Box<Body>),
    Expression(Expression),
    Dummy,
}


/**
 * Defines function in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub original: String,
    pub identifier: String,
    pub parameters: Vec<(String, TypeDecleration)>,
    pub return_type: TypeDecleration,
    pub body: Body,
}


/**
 * Defines while in Rubigo. 
 */
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub original: String,
    pub condition: Expression,
    pub body: Body,
}


/**
 * Defines if in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub original: String,
    pub condition: Expression,
    pub if_body: Body,
    pub else_body: Option<Body>,
}


/**
 * Defines let in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub original: String,
    pub identifier: String,
    pub type_dec: TypeDecleration,
    pub value: Expression,
}


/**
 * Defines Assignment in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub original: String,
    pub identifier: String,
    pub value: Expression,
}


/**
 * Defines return in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub original: String,
    pub value: Expression,
}


/**
 * Defines Body in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub original: String,
    pub body: Vec<Statement>,
}


impl Parser {
    /**
     * Parse Statement.
     */
    pub(super) fn parse_statement(&mut self) -> Statement {
        if self.is_tokentype(TokenType::Fn) {
            return self.parse_function();

        } else if self.is_tokentype(TokenType::While) {
            return self.parse_while();

        } else if self.is_tokentype(TokenType::If) {
            return self.parse_if();

        } else if self.is_tokentype(TokenType::Let) {
            return self.parse_let();

        } else if self.is_tokentype(TokenType::Identifier) {
            return self.parse_identifier_statement();

        } else if self.is_tokentype(TokenType::Return) {
            return self.parse_return();

        } else if self.is_tokentype(TokenType::BodyStart) {
            return self.parse_body();

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Statement".to_string());
            
            let _token: Token = self.next_token();
            
            return Statement::Dummy;
        }
    }


    /**
     * Parse function.
     */
    fn parse_function(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();

        let _fn: Token = self.next_token();
        let fn_identifier: Token = self.parse_type(TokenType::Identifier);
        let _start_p: Token = self.parse_type(TokenType::ParenthesisStart);
       
        let mut parameters: Vec<(String, TypeDecleration)> = Vec::new();
        let mut until: bool = true;
        while until {
            let token: Token = self.next_token();
            match & token.get_type() {
                TokenType::ParenthesisEnd => until = false,
                _ => {
                    let _type_dec: Token = self.parse_type(TokenType::TypeDec);
                    let type_dec: TypeDecleration = self.parse_type_decleration();
                    parameters.push((token.get_value(), type_dec));

                    if self.is_tokentype(TokenType::Comma) {
                        let _comma: Token = self.next_token();
                    }
                },
            };
        }

        let _arrow: Token = self.parse_type(TokenType::FnType);
        let return_type: TypeDecleration = self.parse_type_decleration();
        let body: Body;
        if let Statement::Body(box_body) = self.parse_body() {
            body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            body = Body {
                original: "".to_string(),
                body: Vec::new(), 
            };
        }

        return Statement::Function(Box::new(Function {
            original: self.get_original(original_start),
            identifier: fn_identifier.get_value(),
            parameters: parameters,
            return_type: return_type,
            body: body,
        }));
    }


    /**
     * Parse while.
     */
    fn parse_while(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let _while: Token = self.next_token();
        let expression: Expression = self.parse_expression();
        
        let body: Body;
        if let Statement::Body(box_body) = self.parse_body() {
            body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            body = Body {
                original: "".to_string(),
                body: Vec::new(),
            };
        }

        return Statement::While(Box::new(While {
            original: self.get_original(original_start),
            condition: expression,
            body: body,
        }));
    }


    /**
     * Parse if.
     */
    fn parse_if(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let _if: Token = self.next_token();
        let expression: Expression = self.parse_expression(); 
        
        let if_body: Body;
        if let Statement::Body(box_body) = self.parse_body() {
            if_body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            if_body = Body {
                original: "".to_string(),
                body: Vec::new(),
            };
        }

        let mut else_body: Option<Body> = None;

        if self.peak().get_type() == TokenType::Else {
            let _else: Token = self.next_token();

            if self.is_tokentype(TokenType::BodyStart) {

                let e_body: Body;
                if let Statement::Body(box_body) = self.parse_body() {
                    e_body = * box_body;

                } else {
                    self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
                    e_body = Body {
                        original: "".to_string(),
                        body: Vec::new(),
                    };
                }

                else_body = Some(e_body);

            } else {
                self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
                else_body = Some(Body {
                    original: "".to_string(),
                    body: Vec::new(),
                });
            }
        }
        
        return Statement::If(Box::new(If {
            original: self.get_original(original_start),
            condition: expression,
            if_body: if_body,
            else_body: else_body,
        }));
    }


    /**
     * Parse let.
     */
    fn parse_let(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let _let: Token = self.next_token();
        let identifier: Token = self.parse_type(TokenType::Identifier);

        let _type_dec: Token = self.parse_type(TokenType::TypeDec);
        let type_dec: TypeDecleration = self.parse_type_decleration();
        
        let _equal: Token = self.parse_type(TokenType::Equals);
        let expression: Expression = self.parse_expression();
        let _end: Token = self.parse_type(TokenType::SemiColon);
        
        return Statement::Let(Let {
            original: self.get_original(original_start),
            identifier: identifier.get_value(),
            type_dec: type_dec,
            value: expression, 
        }); 
    }


    /**
     * Parse assignment.
     */
    fn parse_identifier_statement(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let identifier: Token = self.next_token();

        if self.is_tokentype(TokenType::Equals) {
            return self.parse_assignment(identifier, original_start);
        
        } else if self.is_tokentype(TokenType::ParenthesisStart) {
            let statement: Statement = Statement::Expression(self.parse_function_call(identifier));
            let _end: Token = self.parse_type(TokenType::SemiColon);
            return statement;

        } else {
            self.create_error(ErrorLevel::Error,
                              "Expected '=' or '('".to_string());
            return Statement::Dummy;
        }
    }


    /**
     * Parse assignment.
     */
    fn parse_assignment(&mut self, identifier: Token, original_start: usize) -> Statement {
        let _equal: Token = self.next_token();
        let expression: Expression = self.parse_expression();
        let _end: Token = self.parse_type(TokenType::SemiColon);
        
        return Statement::Assignment(Assignment {
            original: self.get_original(original_start),
            identifier: identifier.get_value(),
            value: expression, 
        }); 
    }


    /**
     * Parse return.
     */
    fn parse_return(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let _return: Token = self.next_token();
        let expression: Expression = self.parse_expression();
        let _end: Token = self.parse_type(TokenType::SemiColon);
        
        return Statement::Return(Return {
            original: self.get_original(original_start),
            value: expression, 
        }); 
    }


    /**
     * Parse body.
     */
    fn parse_body(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let _start: Token = self.parse_type(TokenType::BodyStart);
        let mut statements: Vec<Statement> = Vec::new();
         
        loop {
            let token: Token = self.peak();
            match token.get_type() {
                TokenType::BodyEnd => {
                    let _end: Token = self.next_token();

                    return Statement::Body(Box::new(Body {
                        original: self.get_original(original_start),
                        body: statements,
                    }));
                },
                _ => statements.push(self.parse_statement()),
            };
        }
    }
}

