#![allow(dead_code)]

pub use super::{
    Span,
    statement::{
        Function,
        Let,
    },
    value_borrow::BorrowValue,
};

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowEnvironments {
    pub envs: Vec<BorrowEnvironment>,
}

impl BorrowEnvironments {
    pub fn new() -> BorrowEnvironments {
        return BorrowEnvironments {
            envs: vec!(BorrowEnvironment::new()), 
        }; 
    } 

    pub fn create_env(&mut self) {
        self.envs.push(BorrowEnvironment::new());
    }

    pub fn pop_env(&mut self) {
        self.envs.pop();
    }

    pub fn add_variable(&mut self, ident: Span<String>, mutable: bool, original: Span<String>) -> Option<(Span<String>, Span<String>)> {
        //TODO
        None
    }

//    pub fn get_variable(& self, ident: String, env_id: usize) -> Option<(usize, &BorrowVariable)> {
//        match self.envs[env_id].get_variable(&ident) {
//            Some(var) => return Some((env_id, var)),
//            None => {
//                match self.envs[env_id].prev_id {
//                    Some(id) => return self.get_variable(ident, id),
//                    None => return None,
//                };
//            },
//        };
//    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowEnvironment {
    pub variables: HashMap<String, BorrowVariable>, 
    pub stack: Vec<BorrowStack>, 
}


impl BorrowEnvironment {
    pub fn new() -> BorrowEnvironment {
        return BorrowEnvironment{
            variables: HashMap::new(),
            stack: Vec::new(),
        };
    }

    pub fn get_variable(& self, identifier: &String) -> Option<&BorrowVariable> {
        return self.variables.get(identifier);
    }
   

    /**
     * Retruns Borrowvariable if it already is declared and None if it isen't.
     */
    pub fn set_variable(&mut self, variable: BorrowVariable) -> Option<BorrowVariable> {
        match self.get_variable(&variable.get_ident()) {
            Some(var) => return Some(var.clone()),
            None => {
                self.variables.insert(variable.get_ident(), variable);
                return None;
            },
        };
    }    


    pub fn set_value(&mut self, value: BorrowValue, mutable: bool) -> (usize, usize) {
        let stack = BorrowStack{
            value: value.clone(),
            stack: vec!(mutable),
        };
        self.stack.push(stack);
        return (self.stack.len() - 1, 0);
    }    
}

#[derive(Debug, Clone, PartialEq)]
pub struct BorrowStack {
    value: BorrowValue,
    stack: Vec<bool>, // true if unique aka borrow mute and false if shared aka borrow.
}

impl BorrowStack {
    pub fn add(&mut self, mutable: bool) -> usize {
        self.stack.push(mutable);
        return self.stack.len() - 1;
    }

    fn r#use(&mut self, id: usize) -> Option<String> {
        if id >= self.stack.len() {
            return Some("Lifetime had expired".to_string());
        }
        if self.stack[id] {
            while id < self.stack.len() - 1 {
                self.stack.pop();
            }
        }
        return None;
    }

    pub fn update_value(&mut self, value: BorrowValue, id: usize) -> Option<String> {
        if let Some(err) = self.r#use(id) {
            return Some(err);
        }
        if !self.stack[id] {
            panic!("Fatal error: Implementeation is not correct");
        }
        self.value = value;
        return None;
    }

    pub fn get_value(&mut self, id: usize) -> (BorrowValue, Option<String>) {
        return (self.value.clone(), self.r#use(id));
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowVariable {
    pub original: Span<String>,
    pub ident: Span<String>,

    pub pointer: (usize, usize),
    pub mutable: bool,
}

impl BorrowVariable {
    pub fn get_ident(& self) -> String {
        return self.ident.get_fragment();
    }
}

