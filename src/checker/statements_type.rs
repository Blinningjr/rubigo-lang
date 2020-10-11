#![allow(dead_code)]


pub use super::{
    Checker,
//  expressions::Expression,
//    TypeDecleration,
    Span,
    ErrorLevel,
};

pub use super::r#type::{
    Type,
};

pub use super::statement::{
    Statement,
    Function,
    While,
    If,
    Let,
    Assignment,
    Return,
    Body,
};


impl Checker {
    pub(super) fn check_statement(&mut self, statement: Statement) -> () { 
        match statement {
//            Statement::Function(function) => self.check_function(*function),
//            Statement::While(r#while) => self.check_while(*r#while),
//            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
//            Statement::Assignment(assignment) => self.check_assignment(assignment),
//            Statement::Return(r#return) => self.check_return(r#return),
//            Statement::Body(body) => self.check_body(*body, true),
//            Statement::Expression(expression) => self.check_expression(expression),
            _ => panic!("Not implemented!"),
        };
    }

//    fn check_function(&mut self, function: Function) -> () {
//    }
//
//    fn check_while(&mut self, while_statement: While) -> () {
//    }
//
//    fn check_if(&mut self, if_statement: If) -> () {
//    }

    fn check_let(&mut self, let_stmt: Let) -> () {
        let original: Span<String> = let_stmt.original.clone();
        self.check_if_unreachable_code(original.clone());
        
        let var_type: Type;
        match Type::parse(&let_stmt.type_dec.r#type.get_fragment(),
                          let_stmt.type_dec.borrow,
                          let_stmt.type_dec.mutable) {
            Some(t) => var_type = t,
            None => {
                panic!("TODO: Add type error");
            },
        };
        self.add_variable(let_stmt.clone(), var_type.clone());

        let expr_type: Type = self.get_expression_type(let_stmt.value.clone(), original.clone()); 
        
        if !var_type.same_type(&expr_type) {
            panic!("TODO: add error");
 //           let (line, offset): (usize, usize) = self.get_expression_location(let_statement.value);
 //           self.create_type_error(ErrorLevel::Error,
 //                                  format!("Variable {} is of type {} got {}",
 //                                          let_statement.identifier.get_fragment(),
 //                                          variable_type.to_string(),
 //                                          expression_type.to_string()),
 //                                  original,
 //                                  line,
 //                                  offset);
        }
    }
    
//    fn check_assignment(&mut self, assignment: Assignment) -> () {
//    }
//
//    fn check_return(&mut self, return_statement: Return) -> () {
//    }
//
//    fn check_body(&mut self, body: Body, create_env: bool) -> () {
//    }
//
//    fn check_expression(&mut self, expression: Expression) -> () {
//    }

    fn check_if_unreachable_code(&mut self, original: Span<String>) -> () {
        if self.get_environment().returns {
            self.create_type_error(ErrorLevel::Warning, "Unreachable code".to_string(), original.clone(), original.line, original.offset);
        }
    }

}


