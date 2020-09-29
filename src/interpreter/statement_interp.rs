#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,
    Span,
    Literal,
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


impl Interpreter {
    pub fn interpret_statement(&mut self, statement: Statement) -> Literal {
        match statement {
            Statement::Function(_) => (),
            Statement::While(r#while) => return self.interpret_while(*r#while),
            Statement::If(r#if) => return self.interpret_if(*r#if),
            Statement::Let(r#let) => self.interpret_let(r#let),
            Statement::Assignment(assignment) => self.interpret_assignment(assignment),
            Statement::Return(r#return) => return self.interpret_return(r#return),
            Statement::Body(body) => return self.interpret_body(*body),
            Statement::Expression(expression) => self.interpret_expression_statement(expression),
            _ => panic!("Not implemented!"),
        };

        return Literal::Dummy;
    }

    pub(super) fn interpret_function(&mut self, function: Function, values: Vec<Literal>) -> Literal {
        self.create_func_env(function.id);
        for i in 0..function.parameters.len() {
            self.store_variable(function.parameters[i].0.clone(), values[i].clone())
        } 
        let result: Literal = self.interpret_body(function.body);
        
        //if result != Literal::Dummy {
//            println!("{:?} returned: {:?}", function.identifier.get_fragment(), result);
        //}

        self.drop_func_env();
        return result;
    }

    fn interpret_while(&mut self, while_statement: While) -> Literal {
        let mut condition: Literal = self.interpret_expression(while_statement.condition.clone());
        while self.get_bool(condition) {
            let value: Literal = self.interpret_body(while_statement.body.clone());
            if value != Literal::Dummy {
                return value;
            }
            condition = self.interpret_expression(while_statement.condition.clone());
        }
        return Literal::Dummy;
    }

    fn interpret_if(&mut self, if_statement: If) -> Literal {
        let condition: Literal = self.interpret_expression(if_statement.condition);
        if self.get_bool(condition) {
            return self.interpret_body(if_statement.if_body);
        } else {
            match if_statement.else_body {
                Some(body) => return self.interpret_body(body),
                None => (),
            };
        }
        return Literal::Dummy;
    }

    fn interpret_let(&mut self, let_statement: Let) -> () {
        let name: Span<String> = let_statement.identifier;
        let value: Literal = self.interpret_expression(let_statement.value);

        self.store_variable(name.clone(), value);
    }
    
    fn interpret_assignment(&mut self, assignment: Assignment) -> () {
        let name: Span<String> = assignment.identifier;
        let value: Literal = self.interpret_expression(assignment.value);

        self.assign_variable(name, value);
    }

    fn interpret_return(&mut self, return_statement: Return) -> Literal {
        return self.interpret_expression(return_statement.value);
    }

    fn interpret_body(&mut self, body: Body) -> Literal {
        self.create_env();
        let statements: Vec<Statement> = body.body;
        for stmt in statements {
            let value: Literal = self.interpret_statement(stmt);
            if value != Literal::Dummy {
                self.drop_env();
                return value;
            }
        } 
        self.drop_env();
        return Literal::Dummy;
    }

    fn interpret_expression_statement(&mut self, expression: Expression) -> () {
        match expression {
            Expression::FunctionCall(f_call) => self.interpret_function_call(*f_call),
            _ => panic!("Fatal Interpreter error"),
        };
    }
}

