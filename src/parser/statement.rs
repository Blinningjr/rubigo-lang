use super::{
    Parser,
    Token,
    TokenType,
    TypeDecleration,
    Expression,
    ErrorLevel,
    Span,
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
    pub id: usize,
    pub original: Span<String>,
    pub identifier: Span<String>,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeDecleration,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub mutable: Option<Span<String>>,
    pub identifier: Span<String>,
    pub type_dec: TypeDecleration,
}

impl Function{
    pub fn create_dummy() -> Function {
        return Function{
            id: 0,
            original: Span::new("DUMMY".to_string(), 0, 0),
            identifier: Span::new("DUMMY".to_string(), 0, 0),
            parameters: vec!(),
            return_type: TypeDecleration{
                borrow: false,
                mutable: false,
                r#type: Span::new("ANY".to_string(), 0, 0),
            },
            body: Body{
                id: 0,
                original: Span::new("DUMMY".to_string(), 0, 0),
                body: vec!(), 
            },
        };
    }
}


/**
 * Defines while in Rubigo. 
 */
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub id: usize,
    pub original: Span<String>,
    pub condition: Expression,
    pub body: Body,
}


/**
 * Defines if in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub id: usize,
    pub original: Span<String>,
    pub condition: Expression,
    pub if_body: Body,
    pub else_body: Option<Body>,
}


/**
 * Defines let in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub id: usize,
    pub original: Span<String>,
    pub mutable: Option<Span<String>>,
    pub identifier: Span<String>,
    pub type_dec: TypeDecleration,
    pub value: Expression,
}


/**
 * Defines Assignment in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub id: usize,
    pub original: Span<String>,
    pub derefrenced: Option<Span<String>>,
    pub identifier: Span<String>,
    pub value: Expression,
}


/**
 * Defines return in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub id: usize,
    pub original: Span<String>,
    pub value: Expression,
}


/**
 * Defines Body in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub id: usize,
    pub original: Span<String>,
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
            return self.parse_body(true);

        } else if self.is_tokentype(TokenType::Star) {
            let original_start: usize = self.get_original_start();
      
            let star: Token = self.next_token();
            let identifier: Token = self.parse_type(TokenType::Identifier);
            
            return self.parse_assignment(identifier, original_start, Some(self.create_span(star.get_value(), &star)));

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
        let func_id: usize = self.func_id;
        self.func_id += 1;

        let prev_body_id: usize = self.body_id;
        let prev_last_id: usize = self.last_id;
        self.body_id = 0;
        

        let original_start: usize = self.get_original_start();

        let _fn: Token = self.next_token();
        let fn_identifier: Token = self.parse_type(TokenType::Identifier);
        let _start_p: Token = self.parse_type(TokenType::ParenthesisStart);
       
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut until: bool = true;
        while until {
            let token: Token = self.next_token();
            match & token.get_type() {
                TokenType::ParenthesisEnd => until = false,
                _ => {
                    let mut mutable: Option<Span<String>> = None;
                    if self.peak().get_type() == TokenType::Mut {
                        let mut_token: Token = self.next_token();
                        mutable = Some(self.create_span(token.get_value(), &token))
                    }
                    let _type_dec: Token = self.parse_type(TokenType::TypeDec);
                    let type_dec: TypeDecleration = self.parse_type_decleration();
                    parameters.push(Parameter{
                        mutable: mutable,
                        identifier: self.create_span(token.get_value(), & token),
                        type_dec: type_dec});

                    if self.is_tokentype(TokenType::Comma) {
                        let _comma: Token = self.next_token();
                    }
                },
            };
        }

        let _arrow: Token = self.parse_type(TokenType::FnType);
        let return_type: TypeDecleration = self.parse_type_decleration();
        let body: Body;
        if let Statement::Body(box_body) = self.parse_body(false) {
            body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            body = Body {
                id: self.body_id,
                original: Span::new("".to_string(), 0, 0),
                body: Vec::new(), 
            };
        }
        
        self.body_id = prev_body_id;
        self.last_id = prev_last_id;

        return Statement::Function(Box::new(Function {
            id: func_id,
            original: self.get_original(original_start),
            identifier: self.create_span(fn_identifier.get_value(), & fn_identifier),
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
        if let Statement::Body(box_body) = self.parse_body(true) {
            body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            body = Body {
                id: self.body_id,
                original: Span::new("".to_string(), 0, 0),
                body: Vec::new(),
            };
        }

        return Statement::While(Box::new(While {
            id: self.body_id,
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
        if let Statement::Body(box_body) = self.parse_body(true) {
            if_body = * box_body;

        } else {
            self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
            if_body = Body {
                id: self.body_id,
                original: Span::new("".to_string(), 0, 0),
                body: Vec::new(),
            };
        }

        let mut else_body: Option<Body> = None;

        if self.peak().get_type() == TokenType::Else {
            let _else: Token = self.next_token();

            if self.is_tokentype(TokenType::BodyStart) {

                let e_body: Body;
                if let Statement::Body(box_body) = self.parse_body(true) {
                    e_body = * box_body;

                } else {
                    self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
                    e_body = Body {
                        id: self.body_id,
                        original: Span::new("".to_string(), 0, 0),
                        body: Vec::new(),
                    };
                }

                else_body = Some(e_body);

            } else {
                self.create_error(ErrorLevel::Error, "Expected a Body".to_string());
                else_body = Some(Body {
                    id: self.body_id,
                    original: Span::new("".to_string(), 0, 0),
                    body: Vec::new(),
                });
            }
        }
        
        return Statement::If(Box::new(If {
            id: self.body_id,
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
       
        let mut mutable: Option<Span<String>> = None;
        if self.is_tokentype(TokenType::Mut) {
            let mut_token: Token = self.next_token();
            mutable = Some(self.create_span(mut_token.get_value(), & mut_token));
        }

        let identifier: Token = self.parse_type(TokenType::Identifier);

        let _type_dec: Token = self.parse_type(TokenType::TypeDec);
        let type_dec: TypeDecleration = self.parse_type_decleration();
        
        let _equal: Token = self.parse_type(TokenType::Equals);
        let expression: Expression = self.parse_expression();
        let _end: Token = self.parse_type(TokenType::SemiColon);
        
        return Statement::Let(Let {
            id: self.body_id,
            original: self.get_original(original_start),
            mutable: mutable,
            identifier: self.create_span(identifier.get_value(), & identifier),
            type_dec: type_dec,
            value: expression, 
        }); 
    }


    /**
     * Parse statement staring with identifier.
     */
    fn parse_identifier_statement(&mut self) -> Statement {
        let original_start: usize = self.get_original_start();
        
        let identifier: Token = self.next_token();

        if self.is_tokentype(TokenType::Equals) {
            return self.parse_assignment(identifier, original_start, None);
        
        } else if self.is_tokentype(TokenType::ParenthesisStart) {
            let statement: Statement = Statement::Expression(
                self.parse_function_call(identifier, original_start));
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
    fn parse_assignment(&mut self, identifier: Token, original_start: usize, derefrenced: Option<Span<String>>) -> Statement {
        let _equal: Token = self.parse_type(TokenType::Equals);
        let expression: Expression = self.parse_expression();
        let _end: Token = self.parse_type(TokenType::SemiColon);
        
        return Statement::Assignment(Assignment {
            id: self.body_id,
            original: self.get_original(original_start),
            derefrenced: derefrenced,
            identifier: self.create_span(identifier.get_value(), & identifier),
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
            id: self.body_id,
            original: self.get_original(original_start),
            value: expression, 
        }); 
    }


    /**
     * Parse body.
     */
    fn parse_body(&mut self, update_id: bool) -> Statement {
        let prev_body_id: usize = self.body_id;
        if update_id {
            self.last_id += 1;
            self.body_id = self.last_id;
        }

        let original_start: usize = self.get_original_start();
        
        let _start: Token = self.parse_type(TokenType::BodyStart);
        let mut statements: Vec<Statement> = Vec::new();
         
        loop {
            let token: Token = self.peak();
            match token.get_type() {
                TokenType::BodyEnd => {
                    let _end: Token = self.next_token();
        
                    if update_id {
                        self.body_id = prev_body_id;
                    }

                    return Statement::Body(Box::new(Body {
                        id: self.body_id,
                        original: self.get_original(original_start),
                        body: statements,
                    }));
                },
                _ => statements.push(self.parse_statement()),
            };
        }
    }
}

