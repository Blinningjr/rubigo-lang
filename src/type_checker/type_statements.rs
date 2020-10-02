#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Statement,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,
};

pub use super::r#type::{
    Type,
    compare_types,
};

pub use super::statement::{
    Function,
    While,
    If,
    Let,
    Assignment,
    Return,
    Body,
};


impl TypeChecker {
    pub(super) fn check_statement(&mut self, statement: Statement) -> () { 
        match statement {
            Statement::Function(function) => self.check_function(*function),
            Statement::While(r#while) => self.check_while(*r#while),
            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
            Statement::Assignment(assignment) => self.check_assignment(assignment),
            Statement::Return(r#return) => self.check_return(r#return),
            Statement::Body(body) => self.check_body(*body, true),
            Statement::Expression(expression) => self.check_expression(expression),
            _ => panic!("Not implemented!"),
        };
    }

    fn check_function(&mut self, function: Function) -> () {
        let current_id = self.modual.current_env_id;
        let current_body_id = self.modual.current_body_id;

        self.new_function_env(function.clone());
        for p in function.parameters {
            self.add_variable(p.0, p.1);
        }

        self.check_body(function.body, false);

        self.check_if_all_bodies_return();

        self.modual.current_env_id = current_id;
        self.modual.current_body_id = current_body_id;
    }

    fn check_while(&mut self, while_statement: While) -> () {
        let original: Span<String> = while_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        let condition_type: Type = self.get_expression_type(while_statement.condition.clone(), original.clone());
        if !compare_types(&condition_type, &Type::Custom("bool".to_string())) {
            let (line, offset): (usize, usize) = self.get_expression_location(while_statement.condition);
            self.create_type_error(ErrorLevel::Error, "type error in while statement.".to_string(), original, line, offset);
        }

        self.check_body(while_statement.body, true);
    }

    fn check_if(&mut self, if_statement: If) -> () {
        let original: Span<String> = if_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        let condition_type: Type = self.get_expression_type(if_statement.condition.clone(), original.clone());
        if !compare_types(&condition_type, &Type::Custom("bool".to_string())) {
            let (line, offset): (usize, usize) = self.get_expression_location(if_statement.condition);
            self.create_type_error(ErrorLevel::Error, "type error in if statement.".to_string(), original, line, offset);
        }

        self.check_body(if_statement.if_body, true);

        match self.modual.current_env_id {
            Some(id) => {
                let env_id: usize = self.modual.environments[id].environments.len() - 1;
                self.modual.environments[id].environments[env_id].if_body = true;
            },
            None => (),
        };

        match if_statement.else_body {
            Some(body) => self.check_body(body, true),
            None => (),
        };
    }

    fn check_let(&mut self, let_statement: Let) -> () {
        let original: Span<String> = let_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        let variable_type: Type = Type::Custom(let_statement.type_dec.r#type.get_fragment());
        self.add_variable(let_statement.identifier, let_statement.type_dec);
        
        let expression_type: Type = self.get_expression_type(let_statement.value.clone(), original.clone()); 
        if !compare_types(&variable_type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(let_statement.value);
            self.create_type_error(ErrorLevel::Error, "type error in let statement.".to_string(), original, line, offset);
        }
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let original: Span<String> = assignment.original;
        self.check_if_unreachable_code(original.clone());
        
        let variable_type: Type = self.lookup_variable(assignment.identifier, original.clone()).r#type;
        
        let expression_type: Type = self.get_expression_type(assignment.value.clone(), original.clone());
        if !compare_types(&variable_type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(assignment.value);
            self.create_type_error(ErrorLevel::Error, "type error: in assignment statement.".to_string(), original, line, offset);
        }
    }

    fn check_return(&mut self, return_statement: Return) -> () {
        let original: Span<String> = return_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        self.get_environment().returns_value = true;
        
        let expression_type: Type = self.get_expression_type(return_statement.value.clone(), original.clone());
        let return_type: Type = Type::Custom(self.get_function().return_type.r#type.get_fragment());

        if !compare_types(&expression_type, &return_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(return_statement.value);
            self.create_type_error(ErrorLevel::Error, "type error: in return statement.".to_string(), original, line, offset);
        }
    }

    fn check_body(&mut self, body: Body, create_env: bool) -> () {
        let current_body_id: usize = self.modual.current_body_id;

        if create_env {
            self.create_body();
        }

        for statement in body.body.iter() {
            self.check_statement(statement.clone());
        } 
        self.modual.current_body_id = current_body_id;
        if self.modual.environments.len() < 1 {
            self.modual.mod_body_id = current_body_id;
        }
    }

    fn check_expression(&mut self, expression: Expression) -> () {
        match &expression {
            Expression::FunctionCall(expr) => {
                let original: Span<String> = expr.original.clone();
                self.check_if_unreachable_code(original.clone());
                self.get_expression_type(expression, original);
            },
            _ => panic!("Fatal Error!!!"),
        };
    }

    fn check_if_unreachable_code(&mut self, original: Span<String>) -> () {
        if self.get_environment().returns_value {
            self.create_type_error(ErrorLevel::Warning, "Unreachable code".to_string(), original.clone(), original.line, original.offset);
        }
    }

}

