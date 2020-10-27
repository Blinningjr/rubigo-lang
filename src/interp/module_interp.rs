#![allow(dead_code)]

pub use super::{
    Span,
    statement::{
        Function,
    },
};

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct InterpModule {
    pub envs: Vec<InterpEnv>,
    pub func_envs: Vec<InterpFuncEnv>,
}

impl InterpModule {
    pub fn new() -> InterpModule {
       return InterpModule{
            envs: vec!(InterpEnv::new()),
            func_envs: vec!(),
       }; 
    }

    pub fn create_func_env(&mut self, ident: Span<String>) -> () {
        self.func_envs.push(InterpFuncEnv::new(ident));        
    }

    pub fn drop_func_env(&mut self) -> () {
        match self.func_envs.pop() {
            Some(_) => (),
            None => panic!("Fatal Interpreter Error"),
        };
    }

    pub fn create_env(&mut self) -> () {
        match self.func_envs.len() {
            0 => self.envs.push(InterpEnv::new()), 
            n => self.func_envs[n-1].create_env(),
        };
    }
 
    pub fn drop_env(&mut self) -> () {
        match self.func_envs.len() {
            0 => {
                match self.envs.pop() {
                    Some(_) => (),
                    None => panic!("fatal intepreter error"),
                };
            },
            n => self.func_envs[n-1].drop_env(),
        };
    } 

    pub fn update_variable(&mut self, identifier: String, value: Value) -> () { 
        match self.func_envs.len() {
            0 => (),
            n => if !self.func_envs[n-1].update_variable(identifier.clone(), value.clone()) {return;},
        }; 
        fn recursive_var_update(module: &mut InterpModule, identifier: String, value: Value, env_id: usize) -> bool {
            match module.envs[env_id].update_variable(identifier.clone(), value.clone()) {
                false => return false,
                true => {
                    if env_id == 0 {
                        return true;
                    }
                    return recursive_var_update(module, identifier, value, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => if recursive_var_update(self, identifier, value, n-1) {panic!("Fatal Interpreter Error");},
        }; 
    }

    pub fn store_variable(&mut self, identifier: String, value: Value) -> () {
        match self.func_envs.len() {
            0 => (),
            n => return self.func_envs[n-1].store_variable(identifier, value),
        }; 
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return self.envs[n-1].store_variable(identifier, value),
        }; 
    } 

    pub fn get_variable(&mut self, identifier: String) -> Value {
        match self.func_envs.len() {
            0 => (),
            n => {
                match self.func_envs[n-1].get_variable(identifier.clone()) {
                    Some(val) => return val,
                    None => (),
                };
            },
        }
        fn recursive_var_get(module: &mut InterpModule, identifier: String, env_id: usize) -> Value {
            match module.envs[env_id].get_variable(identifier.clone()) {
                Some(val) => return val.clone(),
                None => {
                    if env_id == 0 {
                        panic!("Fatal Interpreter Error");
                    }
                    return recursive_var_get(module, identifier, env_id-1);
                },
            };
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_var_get(self, identifier, n-1),
        }; 
    }

    pub fn store_function(&mut self, identifier: String, func: Function) -> () {
        match self.func_envs.len() {
            0 => (),
            n => return self.func_envs[n-1].store_function(identifier, func),
        }; 
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return self.envs[n-1].store_function(identifier, func),
        }; 
    } 

    pub fn get_function(&mut self, identifier: String) -> Function {
        match self.func_envs.len() {
            0 => (),
            n => {
                match self.func_envs[n-1].get_function(identifier.clone()) {
                    Some(val) => return val,
                    None => (),
                };
            },
        }
        fn recursive_func_get(module: &mut InterpModule, identifier: String, env_id: usize) -> Function {
            match module.envs[env_id].get_function(identifier.clone()) {
                Some(val) => return val.clone(),
                None => {
                    if env_id == 0 {
                        panic!("Fatal Interpreter Error");
                    }
                    return recursive_func_get(module, identifier, env_id-1);
                },
            };
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_func_get(self, identifier, n-1),
        }; 
    }


    pub fn search_value_location(&mut self, identifier: String) -> (Option<usize>, usize, usize) { 
        match self.func_envs.len() {
            0 => (),
            n => {
                match self.func_envs[n-1].search_value_location(identifier.clone()) {
                    Some(val) => return (Some(n-1), val.0, val.1),
                    None => (),
                };
            },
        }

        fn recursive_loc_search(module: &mut InterpModule, identifier: String, env_id: usize) -> (Option<usize>, usize, usize) {
            match module.envs[env_id].search_value_location(identifier.clone()) {
                Some(val) => return (None, env_id, val.clone()),
                None => {
                    if env_id == 0 {
                        panic!("Fatal Interpreter Error");
                    }
                    return recursive_loc_search(module, identifier, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_loc_search(self, identifier, n-1),
        };    
    }

    pub fn get_value(&mut self, pointer: Pointer) -> Value {
        return match pointer.func_id {
            Some(f_id) => self.func_envs[f_id].envs[pointer.env_id].memory.get(&pointer.value_id).unwrap().clone(),
            None => self.envs[pointer.env_id].memory.get(&pointer.value_id).unwrap().clone(),
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct InterpFuncEnv {
    pub ident: Span<String>,
    pub envs: Vec<InterpEnv>,
}

impl InterpFuncEnv {
    pub fn new(identifier: Span<String>) -> InterpFuncEnv {
        return InterpFuncEnv{
            ident: identifier,
            envs: vec!(InterpEnv::new()),
        };
    }
    
    pub fn create_env(&mut self) -> () {
        self.envs.push(InterpEnv::new());
    } 
    
    pub fn drop_env(&mut self) -> () {
        self.envs.pop();
    } 

    // returns true if it diden't find the variable.
    pub fn update_variable(&mut self, identifier: String, value: Value) -> bool {
        fn recursive_var_update(func_env: &mut InterpFuncEnv, identifier: String, value: Value, env_id: usize) -> bool {
            match func_env.envs[env_id].update_variable(identifier.clone(), value.clone()) {
                false => return false,
                true => {
                    if env_id == 0 {
                        return true;
                    }
                    return recursive_var_update(func_env, identifier, value, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_var_update(self, identifier, value, n-1),
        }; 
    }

    pub fn store_variable(&mut self, identifier: String, value: Value) -> () {
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => self.envs[n-1].store_variable(identifier, value),
        };
    }

    pub fn get_variable(&mut self, identifier: String) -> Option<Value> {
        fn recursive_var_get(func_env: &mut InterpFuncEnv, identifier: String, env_id: usize) -> Option<Value> {
            match func_env.envs[env_id].get_variable(identifier.clone()) {
                Some(val) => return Some(val.clone()),
                None => {
                    if env_id == 0 {
                        return None;
                    }
                    return recursive_var_get(func_env, identifier, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_var_get(self, identifier, n-1),
        };
    }

    pub fn store_function(&mut self,  identifier: String, func: Function) -> () {
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => self.envs[n-1].store_function(identifier, func),
        };
    }
    
    pub fn get_function(&mut self, identifier: String) -> Option<Function> {
        fn recursive_func_get(func_env: &mut InterpFuncEnv, identifier: String, env_id: usize) -> Option<Function> {
            match func_env.envs[env_id].get_function(identifier.clone()) {
                Some(val) => return Some(val.clone()),
                None => {
                    if env_id == 0 {
                        return None;
                    }
                    return recursive_func_get(func_env, identifier, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_func_get(self, identifier, n-1),
        };
    }

    pub fn search_value_location(&mut self, identifier: String) -> Option<(usize, usize)> {
        fn recursive_loc_search(func_env: &mut InterpFuncEnv, identifier: String, env_id: usize) -> Option<(usize, usize)> {
            match func_env.envs[env_id].search_value_location(identifier.clone()) {
                Some(val) => return Some((env_id, val.clone())),
                None => {
                    if env_id == 0 {
                        return None;
                    }
                    return recursive_loc_search(func_env, identifier, env_id-1);
                },
            }; 
        }
        match self.envs.len() {
            0 => panic!("Fatal Interpreter Error"),
            n => return recursive_loc_search(self, identifier, n-1),
        };    
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct InterpEnv {
    pub funcs: HashMap<String, Function>,
    pub variables: HashMap<String, usize>,
    pub memory: HashMap<usize, Value>,
    pub next_id: usize,
}

impl InterpEnv {
    pub fn new() -> InterpEnv {
        return InterpEnv{
            funcs: HashMap::new(),
            variables: HashMap::new(),
            memory: HashMap::new(),
            next_id: 0,
        };
    }

    // returns true if it coulden't find the variable.
    pub fn update_variable(&mut self, identifier: String, value: Value) -> bool {
        let location: usize = match self.variables.get(&identifier) {
            Some(val) => val.clone(),
            None => return true,
        };
        self.memory.remove(&location);
        self.memory.insert(location, value);
        return false;
    }

    pub fn store_variable(&mut self, identifier: String, value: Value) -> () {
        self.memory.insert(self.next_id, value);
        self.variables.insert(identifier, self.next_id);
        self.next_id += 1;
    }

    pub fn get_variable(&mut self, identifier: String) -> Option<&Value> {
        let location: &usize = match self.variables.get(&identifier) {
            Some(l) => l,
            None => return None,
        };
        return self.memory.get(location).clone();
    }
    
    pub fn store_function(&mut self, identifier: String, func: Function) -> () {
        self.funcs.insert(identifier, func);
    }

    pub fn get_function(&mut self, identifier: String) -> Option<&Function> {
        return self.funcs.get(&identifier).clone();
    }
    
    pub fn search_value_location(&mut self, identifier: String) -> Option<&usize> {
        return self.variables.get(&identifier);
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    F32(f32),
    Bool(bool),
    Char(char),
    String(String),

    Pointer(Pointer),
}

impl Value {
    pub fn string(& self) -> String {
        return match self {
            Value::I32(val) => format!("{}", val),
            Value::F32(val) => format!("{}", val),
            Value::Bool(val) => format!("{}", val),
            Value::Char(val) => format!("{}", val),
            Value::String(val) => format!("{}", val),
           
            Value::Pointer(val) => format!("{:#?}", val),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pointer{
    pub func_id: Option<usize>,
    pub env_id: usize,
    pub value_id: usize,
}

