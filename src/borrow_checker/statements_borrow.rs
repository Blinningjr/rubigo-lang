#![allow(dead_code)]

pub use super::{
    BorrowChecker,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,

    BorrowEnvironments,
    BorrowVariable,
    BorrowValue,
};


pub use super::statement::{
    Statement,
    Function,
    Parameter,
    While,
    If,
    Let,
    Assignment,
    Return,
    Body,
};


impl BorrowChecker {
    pub(super) fn check_statement(&mut self, envs: &mut BorrowEnvironments, statement: Statement) -> () { 
        match statement {
            Statement::Function(function) => self.add_function(*function),
            Statement::While(r#while) => self.check_while(envs, *r#while),
            Statement::If(r#if) => self.check_if(envs, *r#if),
            Statement::Let(r#let) => self.check_let(envs, r#let),
            Statement::Assignment(assignment) => self.check_assignment(envs, assignment),
            Statement::Return(r#return) => self.check_return(envs, r#return),
            Statement::Body(body) => self.check_body(envs.clone(), *body),
            Statement::Expression(expression) => {self.check_expression(envs, expression);}, // TODO: Check Expressions.
            _ => panic!("Not implemented!"),
        };
    }

    pub(super) fn check_function(&mut self, function: Function) -> () {
        let mut envs = self.create_envs();
        for p in function.parameters.iter() {
            let pointer = envs.add_value(match p.type_dec.borrow {
                true => BorrowValue::UnknownPointer,
                false => BorrowValue::Literal(p.type_dec.mutable),
            }, true);
            envs.add_variable(BorrowVariable {
                original: function.original.clone(),
                ident: p.identifier.clone(),
                pointer: pointer,
                mutable: p.mutable != None,
            });
        }
        self.check_body(envs.clone(), function.body);
    }

    fn check_while(&mut self, envs: &mut BorrowEnvironments, while_stmt: While) -> () {
        self.check_expression(envs, while_stmt.condition);
        self.check_body(envs.clone(), while_stmt.body);
    }

    fn check_if(&mut self, envs: &mut BorrowEnvironments, if_stmt: If) -> () {
        self.check_expression(envs, if_stmt.condition);
        self.check_body(envs.clone(), if_stmt.if_body);
        if let Some(body) = if_stmt.else_body {
            self.check_body(envs.clone(), body);
        }
    }

    fn check_let(&mut self, envs: &mut BorrowEnvironments, let_stmt: Let) -> () {
        let value = self.check_expression(envs, let_stmt.value);

        let mutable = let_stmt.mutable != None;
        let pointer = envs.add_value(value, true);
        envs.add_variable(BorrowVariable{
            original: let_stmt.original.clone(),
            ident: let_stmt.identifier,
            mutable: mutable,
            pointer: pointer,
        });
    }
    
    fn check_assignment(&mut self, envs: &mut BorrowEnvironments, assignment: Assignment) -> () {
        let value = self.check_expression(envs, assignment.value);


        let ident = assignment.identifier.get_fragment();
        if assignment.derefrenced != None {
            let (val, _) = envs.get_value(ident).unwrap();
            match val {
                BorrowValue::Pointer(_, env_id, stack_id, borrow_id) => {
                    if let Some(msg) = envs.update_value(value, env_id, stack_id, borrow_id) {
                        self.create_error(msg);
                    }
                },
                BorrowValue::UnknownPointer => panic!("Fatal borrow checker Error"),
                _ => panic!("Fatal type checker error"),
            };
        } else {
            if let Some(msg) = envs.update_variable(ident, value) {
                self.create_error(msg); 
            }
        }
        
        //TODO: update varaible in mem. remove old pointer?
    }

    fn check_return(&mut self, envs: &mut BorrowEnvironments, return_stmt: Return) -> () {
        let _value = self.check_expression(envs, return_stmt.value);
    }

    fn check_body(&mut self, mut envs: BorrowEnvironments, body: Body) -> () {
        envs.create_env();

        for statement in body.body.iter() {
            self.check_statement(&mut envs, statement.clone());
        } 

        envs.pop_env();    
    }
}

