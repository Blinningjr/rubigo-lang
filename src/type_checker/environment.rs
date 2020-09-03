#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub functions: Vec<(String, String)>, 
    pub variables: Vec<(String, String)>, 
}


impl Environment {
    pub fn new() -> Environment {
        return Environment{
            functions: vec!(),
            variables: vec!(),
        };
    }

    pub fn add_variable(&mut self, identifier: String, r#type: String) -> bool {
        self.variables.push((identifier, r#type));
        return true;
    }
}

