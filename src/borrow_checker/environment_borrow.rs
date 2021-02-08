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
            envs: vec!(BorrowEnvironment::new(0)), 
        }; 
    } 

    pub fn create_env(&mut self) {
        let len =self.envs.len();
        self.envs.push(BorrowEnvironment::new(len));
    }

    pub fn pop_env(&mut self) {
        self.envs.pop();
    }

    pub fn add_variable(&mut self, var: BorrowVariable) -> Option<BorrowVariable> {
        let len = self.envs.len() - 1;
        self.envs[len].add_variable(var)
    }
    
    pub fn add_value(&mut self, val: BorrowValue, mutable: bool) -> (usize, usize){
        let len = self.envs.len() - 1;
        self.envs[len].add_value(val, mutable)
    }

    pub fn update_variable(&mut self, ident: String, value: BorrowValue) -> Option<String> {
        for env in self.envs.iter().rev() {
            let id = env.id;
            match env.get_variable(&ident) {
                Some(var) => {
                    let stack_p = var.pointer.0;
                    let stack_b = var.pointer.1;
                    return self.envs[id].stack[stack_p].update_value(value, stack_b);
                },
                None => (),
            };
        } 
        panic!("Fatal type checker error"); 
        
    }

    pub fn update_value(&mut self, value: BorrowValue, env_id: usize, stack_id: usize, borrow_id: usize) -> Option<String> {
        self.envs[env_id].stack[stack_id].update_value(value, borrow_id)
    }



    pub fn create_pointer(&mut self, ident: String, mutable: bool) -> BorrowValue {
        for env in self.envs.iter().rev() {
            let id = env.id;
            match env.get_variable(&ident) {
                Some(var) => {
                    let stack_p = var.pointer.0;
                    let bstack = self.envs[id].stack[stack_p].add(mutable);
                    return BorrowValue::Pointer(mutable, id, stack_p, bstack);
                },
                None => (),
            };
        } 
        panic!("Fatal type checker error"); 
    }

//    pub fn get_variable(& self, ident: String) -> Option<&BorrowVariable> {
//        for env in self.envs.iter().rev() {
//            match env.get_variable(&ident) {
//                Some(val) => return Some(val),
//                None => (),
//            };
//        } 
//        return None;
//    }
    pub fn get_value(&mut self, ident: String) -> Option<(BorrowValue, Option<String>)> {
        for env in self.envs.iter_mut().rev() {
            match env.get_value(&ident) {
                Some(val) => return Some(val),
                None => (),
            };
        } 
        return None;
    }

}


#[derive(Debug, Clone, PartialEq)]
pub struct BorrowEnvironment {
    pub id: usize,
    pub variables: HashMap<String, BorrowVariable>, 
    pub stack: Vec<BorrowStack>, 
}


impl BorrowEnvironment {
    pub fn new(id: usize) -> BorrowEnvironment {
        return BorrowEnvironment{
            id: id,
            variables: HashMap::new(),
            stack: Vec::new(),
        };
    }

    pub fn get_variable(& self, identifier: &String) -> Option<&BorrowVariable> {
        return self.variables.get(identifier);
    }
    
    pub fn get_value(&mut self, identifier: &String) -> Option<(BorrowValue, Option<String>)> {
        if let Some(var) = self.variables.get(identifier) {
            let (val, err) = self.stack[var.pointer.0].get_value(var.pointer.1);
            if val == BorrowValue::UnknownPointer {
                let id = self.stack.len();
                let stack = BorrowStack{
                    value: val.clone(),
                    stack: vec!(true),
                };
                self.stack.push(stack); 
                self.stack[var.pointer.0].value = BorrowValue::Pointer(true, self.id, id, 0);
                return Some(self.stack[var.pointer.0].get_value(var.pointer.1));
            }
            return Some((val, err));
        }
        return None;
    }
   

    /**
     * Retruns Borrowvariable if it already is declared and None if it isen't.
     */
    pub fn add_variable(&mut self, variable: BorrowVariable) -> Option<BorrowVariable> {
        match self.get_variable(&variable.get_ident()) {
            Some(var) => return Some(var.clone()),
            None => {
                self.variables.insert(variable.get_ident(), variable);
                return None;
            },
        };
    }    


    pub fn add_value(&mut self, value: BorrowValue, mutable: bool) -> (usize, usize) {
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
            return Some(format!("ilegal borrow use. Try borrowing as mutable"));
            //panic!("Fatal error: Implementeation is not correct\n\n{:#?}\nid: {:?}", self, id);
        }
        self.value = value;
        return None;
    }

    pub fn get_value(& self, id: usize) -> (BorrowValue, Option<String>) {
        let mut message = None;
        if id >= self.stack.len() {
            message = Some("ilegal use of borrow".to_string());
        } 
        return (self.value.clone(), message);
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

