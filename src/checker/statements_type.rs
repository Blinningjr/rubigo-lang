#![allow(dead_code)]


pub use super::{
    Checker,
    expressions::Expression,
//    TypeDecleration,
    Span,
    ErrorLevel,

    TypeVarMem,
    TypeFunction,
};

pub use super::r#type::{
    Type,
    MyTypes,
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
            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
            Statement::Assignment(assignment) => self.check_assignment(assignment),
            Statement::Return(r#return) => self.check_return(r#return),
            Statement::Body(body) => self.check_body(*body, true),
            Statement::Expression(expression) => self.check_expression(expression),
            _ => panic!("Not implemented!"),
        };
    }

//    fn check_function(&mut self, function: Function) -> () {
//    }
//
//    fn check_while(&mut self, while_statement: While) -> () {
//    }

    fn check_if(&mut self, if_stmt: If) -> () {
        let original: Span<String> = if_stmt.original;
        self.check_if_unreachable_code(original.clone());

        let condition: Type = self.get_expression_type(if_stmt.condition, original.clone());
        let expected: Type = Type::new(MyTypes::Bool);
        
        if !expected.same_type(& condition) {
            panic!("TODO add error"),
        }

        self.check_body(if_stmt.if_body, true);
    }

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
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let original: Span<String> = assignment.original;
        self.check_if_unreachable_code(original.clone());

        let (_, _, mut ass_var) = self.get_variable(assignment.identifier.get_fragment(), original.clone());
        if assignment.derefrenced != None {
            match ass_var {
                TypeVarMem::Var(_var) => {
                    panic!("TODO add error");
                },
                TypeVarMem::Pointer(func_id, env_id, ident, var) => {
                    ass_var = match func_id {
                        Some(id) => self.module.mod_funcs[id].environments.envs[env_id].get_variable(&ident).unwrap().clone(),
                        None => self.module.mod_envs.envs[env_id].get_variable(&ident).unwrap().clone(),
                    };
                },
            };
        }

        let expr_type: Type = self.get_expression_type(assignment.value, original.clone());
        if !ass_var.get_type().same_type(&expr_type) {
            panic!("TODO add type error");
        } 
    }

    fn check_return(&mut self, return_stmt: Return) -> () {
        let original: Span<String> = return_stmt.original;
        self.check_if_unreachable_code(original.clone());

        self.get_environment().returns = true;

        let expr_type: Type = self.get_expression_type(return_stmt.value.clone(), original.clone()); 

        match self.current_func {
            Some(id) => {
                let func: &TypeFunction = &self.module.mod_funcs[id];
                if func.return_type == None { 
                    // TODO: add type error
                    return
                }
                if !func.return_type.clone().unwrap().same_type(& expr_type) {
                    panic!("TODO add type error");
                }
            },
            None => {
                panic!("TODO add type error");
            },
        };

    }

    fn check_body(&mut self, body: Body, create_env: bool) -> () {
        let current_env: usize = self.current_env;

        if create_env {
            self.create_body();
        }

        for statement in body.body.iter() {
            self.check_statement(statement.clone());
        } 

        self.current_env = current_env;
        if self.current_func == None {
            self.current_mod_env = current_env;
        }
    }

    fn check_expression(&mut self, expression: Expression) -> () {
        match &expression {
            Expression::FunctionCall(expr) => {
                let original: Span<String> = expr.original.clone();
                self.check_if_unreachable_code(original.clone());
                self.get_expression_type(expression, original);
            },
            _ => panic!("fatal panic"),
        };
    }

    fn check_if_unreachable_code(&mut self, original: Span<String>) -> () {
        if self.get_environment().returns {
            self.create_type_error(ErrorLevel::Warning, "Unreachable code".to_string(), original.clone(), original.line, original.offset);
        }
    }

}


