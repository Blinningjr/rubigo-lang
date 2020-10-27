#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,
    Span,
    Literal,
    Value,
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
    pub fn interpret_statement(&mut self, stmt: Statement) -> Option<Value> {
        match stmt {
            Statement::Function(func) => self.store_function(*func),
            Statement::While(while_stmt) => return self.interpret_while(*while_stmt),
            Statement::If(if_stmt) => return self.interpret_if(*if_stmt),
            Statement::Let(let_stmt) => self.interpret_let(let_stmt),
            Statement::Assignment(assignment_stmt) => self.interpret_assignment(assignment_stmt),
            Statement::Return(return_stmt) => return self.interpret_return(return_stmt),
            Statement::Body(body) => return self.interpret_body(*body),
            Statement::Expression(expr) => self.interpret_expression_statement(expr),
            _ => panic!("Fatal Interpreter Error"),
        }; 
        return None;
    }

    pub(super) fn interpret_function(&mut self, function: Function, values: Vec<Value>) -> Option<Value> {
        self.create_func_env(function.identifier.clone());
        for i in 0..function.parameters.len() {
            self.store_variable(function.parameters[i].identifier.get_fragment(), values[i].clone())
        } 
        let result: Option<Value> = self.interpret_body(function.body);
        
        //if result != Literal::Dummy {
//            println!("{:?} returned: {:?}", function.identifier.get_fragment(), result);
        //}

        self.drop_func_env();
        return result;
    }

    fn interpret_while(&mut self, while_stmt: While) -> Option<Value> {
        let mut condition: Value = self.interpret_expression(while_stmt.condition.clone());
        while match condition {Value::Bool(b) => b, _ => panic!("Fatal Interpreter Error"),} {
            let value: Option<Value> = self.interpret_body(while_stmt.body.clone());
            match value {
                Some(_) => return value,
                None => (),
            };
            condition = self.interpret_expression(while_stmt.condition.clone());
        }
        return None;
    }

    fn interpret_if(&mut self, if_stmt: If) -> Option<Value> {
        let condition: Value = self.interpret_expression(if_stmt.condition);
        if match condition {Value::Bool(b) => b, _ => panic!("Fatal Interpreter Error"),} {
            return self.interpret_body(if_stmt.if_body);
        } else {
            match if_stmt.else_body {
                Some(body) => return self.interpret_body(body),
                None => (),
            };
        }
        return None;
    }

    fn interpret_let(&mut self, let_stmt: Let) -> () {
        let name: Span<String> = let_stmt.identifier;
        let value: Value = self.interpret_expression(let_stmt.value);

        self.store_variable(name.get_fragment(), value);
    }

    fn interpret_assignment(&mut self, assignment: Assignment) -> () {
       let name: String = assignment.identifier.get_fragment();
       let value: Value = self.interpret_expression(assignment.value);

       self.assign_variable(name, value);
    }
    
    fn interpret_return(&mut self, return_statement: Return) -> Option<Value> {
        return Some(self.interpret_expression(return_statement.value));
    }
    
    fn interpret_body(&mut self, body: Body) -> Option<Value> {
        self.create_env();
        let statements: Vec<Statement> = body.body;
        for stmt in statements {
            let value: Option<Value> = self.interpret_statement(stmt);
            match value {
                Some(_) => {
                    self.drop_env();
                    return value;
                },
                None => (),
            };
        } 
        self.drop_env();
        return None;
    }
    
    fn interpret_expression_statement(&mut self, expression: Expression) -> () {
        match expression {
            Expression::FunctionCall(f_call) => self.interpret_function_call(*f_call),
            _ => panic!("Fatal Interpreter error"),
        };
    }
}

