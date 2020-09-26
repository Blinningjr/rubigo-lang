#![allow(dead_code)]


pub use super::{
    Interpreter,
    Literal,
};

impl Interpreter {
    pub(super) fn get_i32(&mut self, literal: Literal) -> i32 {
        match literal {
            Literal::I32(val) => return val.get_fragment(),
            _ => panic!("Interpreter Type error"),
        };
    }
    
    pub(super) fn get_f32(&mut self, literal: Literal) -> f32 {
        match literal {
            Literal::F32(val) => return val.get_fragment(),
            _ => panic!("Interpreter Type error"),
        }; 
    }
    
    pub(super) fn get_bool(&mut self, literal: Literal) -> bool {
        match literal {
            Literal::Bool(val) => return val.get_fragment(),
            _ => panic!("Interpreter Type error"),
        }; 
    }

    pub(super) fn get_char(&mut self, literal: Literal) -> char {
        match literal {
            Literal::Char(val) => return val.get_fragment(),
            _ => panic!("Interpreter Type error"),
        }; 
    }
    
    pub(super) fn get_string(&mut self, literal: Literal) -> String {
        match literal {
            Literal::String(val) => return val.get_fragment(),
            _ => panic!("Interpreter Type error"),
        }; 
    }
}

