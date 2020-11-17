#![allow(dead_code)]


pub use super::{
    Checker,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,

    TypeFunction,
    TypeVariable,
};

pub use super::r#type::{
    Type,
    MyTypes,
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


impl Checker {
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

    pub(super) fn check_function(&mut self, function: Function) -> () {
        let original: Span<String> = function.original.clone();
        let current_env = self.current_env;
        let current_func = self.current_func;

        let mut parameters: Vec<(bool, Type)> = vec!();
        for p in function.parameters.clone() {
            parameters.push(self.get_parameter_type(p, original.clone()));
        }

        let return_type: Option<Type> = self.get_type_dec_type(function.return_type.clone());
        
        self.new_function_env(function.clone(), parameters.clone(), return_type);

        for i in 0..parameters.len() {
            self.add_variable(original.clone(), function.parameters[i].identifier.clone(), parameters[i].0, parameters[i].1.clone());
        }

        self.check_body(function.body, false);

        self.check_if_all_bodies_return();

        self.current_env = current_env;
        self.current_func = current_func;
    }

    fn get_parameter_type(&mut self, parameter: Parameter, original: Span<String>) -> (bool, Type) {
        match self.get_type_dec_type(parameter.type_dec.clone()) {
            Some(t) => return (parameter.mutable != None, t),
            None => {
                self.create_type_error(ErrorLevel::Error,
                                       format!("{} is not a supported type", parameter.type_dec.r#type.get_fragment()),
                                       original,
                                       parameter.identifier.get_line(),
                                       parameter.identifier.get_offset());
                return (true, Type{
                    ident: None,
                    borrow: false,
                    mutable: false,
                    location: self.get_location(),
                    r#type: MyTypes::Any,
                });
            },
        };
    }

    fn get_type_dec_type(&mut self, type_dec: TypeDecleration) -> Option<Type> {
       return Type::parse(&type_dec.r#type.get_fragment(), type_dec.borrow, type_dec.mutable, self.get_location());
    }

    fn check_while(&mut self, while_stmt: While) -> () {
        let original: Span<String> = while_stmt.original;
        self.check_if_unreachable_code(original.clone());

        let condition: Type = self.get_expression_type(while_stmt.condition.clone(), original.clone());
        let expected: Type = Type::new(MyTypes::Bool, self.get_location());
        
        if !expected.same_type(& condition) {
            let (line, offset): (usize, usize) = self.get_expression_location(while_stmt.condition);
            self.create_type_error(ErrorLevel::Error,
                                  format!("Expected type {} got {}", expected.to_string(), condition.to_string()),
                                  original.clone(),
                                  line,
                                  offset);
        }

        self.check_body(while_stmt.body, true);
    }

    fn check_if(&mut self, if_stmt: If) -> () {
        let original: Span<String> = if_stmt.original;
        self.check_if_unreachable_code(original.clone());

        let condition: Type = self.get_expression_type(if_stmt.condition.clone(), original.clone());
        let expected: Type = Type::new(MyTypes::Bool, self.get_location());
        
        if !expected.same_type(& condition) {
            let (line, offset): (usize, usize) = self.get_expression_location(if_stmt.condition);
            self.create_type_error(ErrorLevel::Error,
                                  format!("Expected type {} got {}", expected.to_string(), condition.to_string()),
                                  original.clone(),
                                  line,
                                  offset);
        }

        self.check_body(if_stmt.if_body, true);

        self.set_is_if_body();
        
        match if_stmt.else_body {
            Some(body) => {
                self.check_body(body, true);
                self.set_is_if_body();
            },
            None => (),
        };
    }

    fn check_let(&mut self, let_stmt: Let) -> () {
        let original: Span<String> = let_stmt.original.clone();
        self.check_if_unreachable_code(original.clone());
        
        let mut var_type: Type = Type{
            ident: None,
            borrow: false,
            mutable: false,
            location: self.get_location(),
            r#type: MyTypes::Any,
        };
        match Type::parse(&let_stmt.type_dec.r#type.get_fragment(),
                          let_stmt.type_dec.borrow,
                          let_stmt.type_dec.mutable,
                          self.get_location()) {
            Some(t) => var_type = t,
            None => {
                self.create_type_error(ErrorLevel::Error,
                                      format!("Type {} not supported", let_stmt.type_dec.r#type.get_fragment()),
                                      original.clone(),
                                      let_stmt.type_dec.r#type.get_line(),
                                      let_stmt.type_dec.r#type.get_offset());
            },
        };

        let expr_type: Type = self.get_expression_type(let_stmt.value.clone(), original.clone()); 
        if var_type.borrow {
            var_type.ident = expr_type.ident.clone();
            self.check_borrow_scope(var_type.location, expr_type.location, let_stmt.identifier.clone(), original.clone());
        }

        self.add_variable(original.clone(), let_stmt.identifier.clone(), let_stmt.mutable != None, var_type.clone());
        
        if !var_type.same_type(&expr_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(let_stmt.value);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Variable {} is of type {} got {}",
                                           let_stmt.identifier.get_fragment(),
                                           var_type.to_string(),
                                           expr_type.to_string()),
                                   original,
                                   line,
                                   offset);
        }
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let original: Span<String> = assignment.original;
        self.check_if_unreachable_code(original.clone());

        let expr_type: Type = self.get_expression_type(assignment.value.clone(), original.clone());
        
        let (_, _, mut ass_var) = match self.get_variable(assignment.identifier.get_fragment(), original.clone()) {
            Some(val) => val,
            None => return,
        };
        if assignment.derefrenced != None {
            if !ass_var.r#type.borrow {
                self.create_borrow_error(ErrorLevel::Error,
                                      format!("Can't derefrence none borrowed value"),
                                      original.clone(),
                                      assignment.derefrenced.clone().unwrap().get_line(),
                                      assignment.derefrenced.unwrap().get_offset());
            } else if !ass_var.r#type.mutable {
                self.create_borrow_error(ErrorLevel::Error,
                                      format!("Can't mutate none mutable borrowed value"),
                                      original.clone(),
                                      assignment.identifier.get_line(),
                                      assignment.identifier.get_offset());
            }
            ass_var.r#type.borrow = false;
            ass_var.r#type.mutable = false;
        } else if ass_var.r#type.borrow {
            self.check_borrow_scope(ass_var.r#type.location, expr_type.location, assignment.identifier.clone(), original.clone());
            match ass_var.r#type.ident {
                Some(ident) => {
                    if ass_var.r#type.mutable {
                        self.remove_borrow_as_mut(ident);
                    } else {
                        self.remove_borrow(ident);
                    }
                },
                None => (),
            };
            ass_var.r#type.ident = expr_type.ident.clone();
        
            if !ass_var.mutable {
                self.create_borrow_error(ErrorLevel::Error,
                                      format!("Can't mutate none mutable borrowed value"),
                                      original.clone(),
                                      assignment.identifier.get_line(),
                                      assignment.identifier.get_offset());
            }
        } else {
            if !ass_var.mutable {
                self.create_borrow_error(ErrorLevel::Error,
                                      format!("Can't mutate none mutable borrowed value"),
                                      original.clone(),
                                      assignment.identifier.get_line(),
                                      assignment.identifier.get_offset());
            }
        }

        if !ass_var.r#type.same_type(&expr_type) {
            let (line, offset): (usize, usize) = self.get_expression_location(assignment.value);
            self.create_type_error(ErrorLevel::Error,
                                   format!("Variable {} is of type {} got {}",
                                           assignment.identifier.get_fragment(),
                                           ass_var.r#type.to_string(),
                                           expr_type.to_string()),
                                   original,
                                   line,
                                   offset);
        } 
    }

    fn check_return(&mut self, return_stmt: Return) -> () {
        let original: Span<String> = return_stmt.original;
        self.check_if_unreachable_code(original.clone());

        self.get_environment().returns = true;

        let expr_type: Type = self.get_expression_type(return_stmt.value.clone(), original.clone()); 

        match self.current_func {
            Some(id) => {
                let func: TypeFunction = self.module.mod_funcs[id].clone();
                if func.return_type == None { 
                    let (line, offset): (usize, usize) = self.get_expression_location(return_stmt.value);
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Function {} dosen't return", func.og_func.identifier.get_fragment()),
                                           original,
                                           line,
                                           offset);
                    return;
                }
                if !func.return_type.clone().unwrap().same_type(& expr_type) {
                    let (line, offset): (usize, usize) = self.get_expression_location(return_stmt.value);
                    self.create_type_error(ErrorLevel::Error,
                                           format!("Expected type {} got {}", func.return_type.clone().unwrap().to_string(), expr_type.to_string()),
                                           original,
                                           line,
                                           offset);
                }
            },
            None => {
                let (line, offset): (usize, usize) = self.get_expression_location(return_stmt.value);
                self.create_type_error(ErrorLevel::Error,
                                       format!("Can't return outside of function"),
                                       original,
                                       line,
                                       offset);
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

        self.remove_all_used_in_current_env();

        self.current_env = current_env;
        if self.current_func == None {
            self.current_mod_env = current_env;
        }
    }

    fn check_expression(&mut self, expression: Expression) -> () {
        match &expression {
            Expression::FunctionCall(expr) => {
                let original: Span<String> = (**expr).original.clone();
                self.check_if_unreachable_code(original.clone());
                self.get_function_call_type((**expr).clone(), original);
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


