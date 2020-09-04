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

    pub fn add_function(&mut self, identifier: String, r#type: String) -> bool {
        self.functions.push((identifier, r#type));
        return true;
    }

    pub fn lookup_function(&mut self, identifier: String) -> Result<String, String> {
        for (ident, r#type) in self.functions.iter() {
            if *ident == identifier {
                return Ok(r#type.clone());
            }
        }
        return Err(format!("Function {:#?} not in scope.", identifier));
    }

    pub fn add_variable(&mut self, identifier: String, r#type: String) -> bool {
        self.variables.push((identifier, r#type));
        return true;
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<String, String> {
        for (ident, r#type) in self.variables.iter() {
            if *ident == identifier {
                return Ok(r#type.clone());
            }
        }
        return Err(format!("Variable {:#?} not in scope.", identifier));
    }
}

