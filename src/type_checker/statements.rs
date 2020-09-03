#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Statement,
    TypeDecleration,
};

pub use super::statement::{
    Let,
};


impl TypeChecker {
    pub(super) fn check_let(&mut self, let_statement: Let) -> () {
        let variable_type: String = let_statement.type_dec.r#type.get_fragment();
        self.add_variable(let_statement.identifier.get_fragment(), variable_type.clone());
        
        let expression_type: String = self.get_expression_type(let_statement.value);
        if variable_type != expression_type {
            self.create_error(let_statement.identifier.get_fragment());
        }
    }
}

