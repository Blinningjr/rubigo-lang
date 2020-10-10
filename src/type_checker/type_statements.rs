#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Statement,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,
    Variable,
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
            self.add_variable(p.identifier, p.type_dec, p.mutable);
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
        if !compare_types(&condition_type, &Type::Custom("bool".to_string(), false, false)) {
            let (line, offset): (usize, usize) = self.get_expression_location(while_statement.condition);
            self.create_type_error(ErrorLevel::Error,
                                   format!("while condition must be of type bool got {}", condition_type.to_string()),
                                   original,
                                   line,
                                   offset);
        }

        self.check_body(while_statement.body, true);
    }

    fn check_if(&mut self, if_statement: If) -> () {
        let original: Span<String> = if_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        let condition_type: Type = self.get_expression_type(if_statement.condition.clone(), original.clone());
        if !compare_types(&condition_type, &Type::Custom("bool".to_string(), false, false)) {
            let (line, offset): (usize, usize) = self.get_expression_location(if_statement.condition);
            self.create_type_error(ErrorLevel::Error,
                                   format!("if condition must be of type bool got {}", condition_type.to_string()),
                                   original,
                                   line,
                                   offset);
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
        
        let variable_type: Type = Type::Custom(let_statement.type_dec.r#type.get_fragment(), let_statement.type_dec.borrow, let_statement.type_dec.mutable);
        self.add_variable(let_statement.identifier.clone(), let_statement.type_dec, let_statement.mutable.get_fragment());
        
        let expression_type: Type = self.get_expression_type(let_statement.value.clone(), original.clone()); 
        if !compare_types(&variable_type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(let_statement.value);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Variable {} is of type {} got {}",
                                           let_statement.identifier.get_fragment(),
                                           variable_type.to_string(),
                                           expression_type.to_string()),
                                   original,
                                   line,
                                   offset);
        }
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let original: Span<String> = assignment.original;
        self.check_if_unreachable_code(original.clone());
        
        let mut variable: Variable = self.lookup_variable(assignment.identifier.clone(), original.clone());

        if assignment.derefrenced.get_fragment() {
            let mut isMutable: bool = false; 
            let mut borrowed: bool = false;
            match &variable.r#type {
                Type::Any => {
                    borrowed = true;
                    isMutable = true;
                },
                Type::Number(b, m) => {
                    isMutable = *m;
                    borrowed = *b;
                    variable.r#type = Type::Number(false, false);
                },
                Type::Custom(t, b, m) => {
                    isMutable = *m;
                    borrowed = *b;
                    variable.r#type = Type::Custom(t.to_string(), false, false);
                },
            };
            if !borrowed  {
                self.create_type_error(ErrorLevel::Error,
                                       format!("Variable {} can't derefrence none borrowed value", assignment.identifier.get_fragment()),
                                       original.clone(),
                                       assignment.identifier.get_line(),
                                       assignment.identifier.get_offset());

            } else if !isMutable {
                self.create_type_error(ErrorLevel::Error,
                                       format!("Variable {} is not borrowed as mutable", assignment.identifier.get_fragment()),
                                       original.clone(),
                                       assignment.identifier.get_line(),
                                       assignment.identifier.get_offset());

            }

        } else if !variable.mutable {
            self.create_type_error(ErrorLevel::Error,
                                   format!("Variable {} is not mutable", assignment.identifier.get_fragment()),
                                   original.clone(),
                                   assignment.identifier.get_line(),
                                   assignment.identifier.get_offset());
        }



        let expression_type: Type = self.get_expression_type(assignment.value.clone(), original.clone());
        if !compare_types(&variable.r#type, &expression_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(assignment.value);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Variable {} is of type {} got {}",
                                           assignment.identifier.get_fragment(),
                                           variable.r#type.to_string(),
                                           expression_type.to_string()),
                                   original,
                                   line,
                                   offset);
        }
    }

    fn check_return(&mut self, return_statement: Return) -> () {
        let original: Span<String> = return_statement.original;
        self.check_if_unreachable_code(original.clone());
        
        self.get_environment().returns_value = true;
        
        let expression_type: Type = self.get_expression_type(return_statement.value.clone(), original.clone());
        let func_return: TypeDecleration = self.get_function().return_type;
        let return_type: Type = Type::Custom(func_return.r#type.get_fragment(), func_return.borrow, func_return.mutable);

        if !compare_types(&expression_type, &return_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(return_statement.value);
            let func_name: String = self.get_function().identifier.get_fragment();
            self.create_type_error(ErrorLevel::Error,
                                   format!("Function {} has return type {} got {}",
                                           func_name,
                                           return_type.to_string(),
                                           expression_type.to_string()),
                                   original,
                                   line,
                                   offset);
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

