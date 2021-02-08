#![allow(dead_code)]

pub use super::{
    Span,
    r#type::Type,
    statement::{
        Function,
        Let,
    },
};

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnvironments {
    pub envs: Vec<TypeEnvironment>,
}

impl TypeEnvironments {
    pub fn new() -> TypeEnvironments {
        return TypeEnvironments {
            envs: vec!(TypeEnvironment::new(0, None)), 
        }; } 
    pub fn get_func(&mut self, id: usize) -> &mut TypeEnvironment {
        return &mut self.envs[id];
    }

    pub fn get_variable(& self, ident: String, env_id: usize) -> Option<(usize, &TypeVariable)> {
        match self.envs[env_id].get_variable(&ident) {
            Some(var) => return Some((env_id, var)),
            None => {
                match self.envs[env_id].prev_id {
                    Some(id) => return self.get_variable(ident, id),
                    None => return None,
                };
            },
        };
    }


    pub fn get_function_id(& self, ident: String, env_id: usize) -> Option<usize> {
        match self.envs[env_id].get_function_id(&ident) {
            Some(func_id) => return Some(func_id),
            None => {
                match self.envs[env_id].prev_id {
                    Some(id) => return self.envs[id].get_function_id(&ident),
                    None => return None,
                };
            },
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnvironment {
    pub id: usize,
    pub prev_id: Option<usize>,

    pub functions: HashMap<String, usize>, 
    pub variables: HashMap<String, TypeVariable>, 

    pub returns: bool,
    pub if_body: bool,
    pub else_body: bool,
}


impl TypeEnvironment {
    pub fn new(id: usize, prev_id: Option<usize>) -> TypeEnvironment {
        return TypeEnvironment{
            id: id,
            prev_id: prev_id,

            functions: HashMap::new(),
            variables: HashMap::new(),

            returns: false,
            if_body: false,
            else_body: false,
        };
    }

    pub fn get_variable(& self, identifier: &String) -> Option<&TypeVariable> {
        return self.variables.get(identifier);
    }
   

    /**
     * Returns Type variable if it already is declared and None if it isn't.
     */
    pub fn set_variable(&mut self, variable: TypeVariable) -> Option<TypeVariable> {
        match self.get_variable(&variable.get_ident()) {
            Some(var) => return Some(var.clone()),
            None => {
                self.variables.insert(variable.get_ident(), variable);
                return None;
            },
        };
    }


    pub fn get_function_id(& self, identifier: &String) -> Option<usize> {
        return match self.functions.get(identifier) {
            Some(id) => Some(*id),
            None => None,
        };
    }
   

    /**
     * Returns Type Function if it already is declared and None if it isn't.
     */
    pub fn set_function(&mut self, function: TypeFunction) -> Option<usize> {
        match self.get_function_id(&function.get_ident()) {
            Some(func_id) => return Some(func_id),
            None => {
                self.functions.insert(function.get_ident(), function.get_id());
                return None;
            },
        };
    }

    pub fn add_borrow_as_mut(&mut self, ident: &String) -> Result<bool, TypeVariable> {
        let mut var: TypeVariable = self.get_variable(ident).unwrap().clone();
        self.variables.remove(ident);

        let result: bool = var.add_borrow_as_mut();
        if self.set_variable(var.clone()) != None {
            panic!("Fatal error");
        }

        if result {
            return Err(var.clone());
        } else {
            return Ok(true);
        }
    }
    
    pub fn add_borrow(&mut self, ident: &String) -> Result<bool, TypeVariable> {
        let mut var: TypeVariable = self.get_variable(ident).unwrap().clone();
        self.variables.remove(ident);

        let result: bool = var.add_borrow();
        if self.set_variable(var.clone()) != None {
            panic!("Fatal error");
        }

        if result {
            return Err(var.clone());
        } else {
            return Ok(true);
        }
    }


    pub fn remove_borrow_as_mut(&mut self, ident: &String) -> () {
        let mut var: TypeVariable = self.get_variable(ident).unwrap().clone();
        self.variables.remove(ident);

        var.remove_borrow_as_mut();
        if self.set_variable(var.clone()) != None {
            panic!("Fatal error");
        }
    }

    pub fn remove_borrow(&mut self, ident: &String) -> () {
        let mut var: TypeVariable = self.get_variable(ident).unwrap().clone();
        self.variables.remove(ident);

        var.remove_borrow();
        if self.set_variable(var.clone()) != None {
            panic!("Fatal error");
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunction {
    pub og_func: Function, 

    // The bool is for mutability
    pub parameters: Vec<(bool, Type)>,

    pub environments: TypeEnvironments,

    pub return_type: Option<Type>,
}

impl TypeFunction {
    pub fn new(func: Function, parameters: Vec<(bool, Type)>, return_type: Option<Type>) -> TypeFunction {
        return TypeFunction{
            og_func: func,

            parameters: parameters,

            environments: TypeEnvironments::new(),

            return_type: return_type,
        };
    }

    pub fn get_ident(& self) -> String {
        return self.og_func.identifier.get_fragment();
    }
    
    pub fn get_id(& self) -> usize {
        return self.og_func.id;
    }

    pub fn get_variable(& self, ident: String, env_id: usize) -> Option<(usize, &TypeVariable)> {
        return self.environments.get_variable(ident, env_id)
    }

    pub fn get_function_id(& self, ident: String, env_id: usize) -> Option<usize> {
        return self.environments.get_function_id(ident, env_id)
    }
   
    pub fn create_env(&mut self, id: usize) -> usize {
        let new_id: usize = self.environments.envs.len();
        self.environments.envs.push(TypeEnvironment::new(new_id, Some(id)));
        return new_id;
    }

    pub fn check_if_all_bodies_return(&mut self) -> bool {
        return match self.return_type {
            Some(_) => self.check_if_all_returns(0),
            None => true,
        };
    }
    
    fn check_if_all_returns(& self, env_id: usize) -> bool {
        if self.environments.envs[env_id].returns {
            return true;
        } 

        let (if_children, _non_if_children): (Vec<usize>, Vec<usize>) = self.separate_if_env(self.find_childrens_ids(env_id));
        
        
        for i in 0..if_children.len() {
            if self.environments.envs[if_children[i]].else_body {
                if self.check_if_all_returns(if_children[i - 1]) &&
                    self.check_if_all_returns(if_children[i]) {
                    return true;
                }   
            }
        } 
        return false;
    }

    fn find_childrens_ids(& self, parent_id: usize) -> Vec<usize> {
        let mut childrens_ids: Vec<usize> = vec!();
        for env in self.environments.envs.iter() {
            match env.prev_id {
                Some(id) => {
                    if id == parent_id {
                        childrens_ids.push(env.id); 
                    }
                },
                None => (),
            };
        }
        return childrens_ids;
    }

    /*
     * Returns (if_envs, non_if_envs)
     */
    fn separate_if_env(& self, envs: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        let mut if_children: Vec<usize> = vec!();
        let mut non_if_children: Vec<usize> = vec!();
        for id in envs {
            if self.environments.envs[id].if_body || self.environments.envs[id].else_body {
                if_children.push(id);
            } else {
                non_if_children.push(id);
            }
        }
        return (if_children, non_if_children);
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TypeVariable {
    pub original: Span<String>,
    pub ident: Span<String>,

    pub mutable: bool,
    pub r#type: Type,

    pub num_borrows: usize,
    pub borrowed_as_mut: bool,
}

impl TypeVariable {
    pub fn get_ident(& self) -> String {
        return self.ident.get_fragment();
    }

    // returns true if error
    pub fn check_borrow_error(& self) -> bool {
        return self.num_borrows > 0 && self.borrowed_as_mut;
    }

    // returns true if error
    pub fn add_borrow(&mut self) -> bool{
        self.num_borrows += 1;
        return self.check_borrow_error();
    }
    
    pub fn remove_borrow(&mut self) -> (){
        if self.num_borrows < 1 {
            panic!("Fatal checker error");
        }
        self.num_borrows -= 1;
    }

    // Returns true if it already is borrowed as mutable
    pub fn add_borrow_as_mut(&mut self) -> bool {
        if self.borrowed_as_mut {
            return true;
        }
        self.borrowed_as_mut = true;
        return false;
    }
    
    // Returns true if it already is borrowed as mutable
    pub fn remove_borrow_as_mut(&mut self) -> () {
        if !self.borrowed_as_mut {
            panic!("Fatal checker error");
        }
        self.borrowed_as_mut = false;
    }
}

