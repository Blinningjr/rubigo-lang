#![allow(dead_code)]

use super::span::Span;

/**
 * Defines error levels in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorLevel {
    Critical,
    Error,
    Warning,
}


/**
 * Defines Error in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Error(String),
    SyntaxError(SyntaxError),
    TypeError(TypeError),
    BorrowError(BorrowError),
}

/**
 * Defines Error in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError {
    pub level: ErrorLevel,
    pub message: String,
    pub code: String,
    pub line: usize,
    pub offset: usize,
}

/**
 * Defines Type Error in Rubgio.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub level: ErrorLevel,
    pub message: String,
    pub code: Span<String>,
    pub line: usize,
    pub offset: usize,
}

/**
 * Defines Borrow Error in Rubgio.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct BorrowError {
    pub level: ErrorLevel,
    pub message: String,
    pub code: Span<String>,
    pub line: usize,
    pub offset: usize,
}

/**
 * Defines error handler in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorHandler {
    errors: Vec<Error>,
    verbose: bool,
}


impl ErrorHandler {
    /**
     * Creates a new Error Handler.
     */
    pub fn new(verbose: bool) -> ErrorHandler {
        return ErrorHandler {
            errors: Vec::new(),
            verbose: verbose,
        };
    }

    pub fn get_num_errors(& self) -> usize {
        return self.errors.len();
    }


    /**
     * Add Error..
     */
    pub fn add(&mut self, error: Error) -> () {
        self.errors.push(error.clone());

        match error {
            Error::SyntaxError(err) => {    
                if err.level == ErrorLevel::Critical {
                    self.print_errors();
                }
            },
            _ => (),
        }  
    }
    

    /**
     * Print Errors.
     */
    pub fn print_errors(&mut self) -> () {
        if self.errors.len() > 0 {
            for err in self.errors.clone() {
                self.print_error(err);
            }

            panic!();
        } 
    }


    /**
     * Print Error.
     */
    pub fn print_error(&mut self, error: Error) -> () {
        match error {
            Error::Error(message) => println!("Error \n\t{:?}\n", message),
            Error::SyntaxError(err) => self.print_syntax_error(err),
            Error::TypeError(err) => self.print_type_error(err),
            Error::BorrowError(err) => self.print_borrow_error(err),
        };
    }


    /**
     * Print Syntax Error.
     */
    fn print_syntax_error(&mut self, error: SyntaxError) -> () {
        let level: String; match &error.level {
            ErrorLevel::Critical => level = "Critical Syntax Error".to_string(),
            ErrorLevel::Error => level = "Syntax Error".to_string(),
            ErrorLevel::Warning => level = "Warning".to_string(),
        };

        let mut location: String = format!(" at {:?}:{:?}", error.line, error.offset).to_string();
        if error.line == 0 && error.offset == 0 {
            location =  "".to_string();
        }

        let line_num: String = format!("{} |", error.line);

        let mut pointer: String = "".to_string();

        for _i in 1..(error.offset+line_num.len()) {
            pointer.push('.');
        }
        pointer.push('▲');

        println!("{}{}", level, location);
        println!("{}{:#}", line_num, error.code);
        println!("{} {}\n", pointer, error.message);
    }


    /**
     * Print Type Error.
     */
    fn print_type_error(&mut self, error: TypeError) -> () {
        let level: String; match &error.level {
            ErrorLevel::Critical => level = "Critical Type Error".to_string(),
            ErrorLevel::Error => level = "Type Error".to_string(),
            ErrorLevel::Warning => level = "Type Warning".to_string(),
        };
        
        let mut location: String = format!(" at {:?}:{:?}", error.line, error.offset).to_string();
        if error.line == 0 && error.offset == 0 {
            location =  "".to_string();
        }

        println!("{}{}\n\t{}", level, location, error.message); 
        println!("Line {}, Code :\n{:#}\n", error.code.get_line(), error.code.get_fragment());
    }
    
    /**
     * Print Borrow Error.
     */
    fn print_borrow_error(&mut self, error: BorrowError) -> () {
        let level: String; match &error.level {
            ErrorLevel::Critical => level = "Critical Borrow Error".to_string(),
            ErrorLevel::Error => level = "Borrow Error".to_string(),
            ErrorLevel::Warning => level = "Borrow Warning".to_string(),
        };
        
        let mut location: String = format!(" at {:?}:{:?}", error.line, error.offset).to_string();
        if error.line == 0 && error.offset == 0 {
            location =  "".to_string();
        }

        println!("{}{}\n\t{}", level, location, error.message); 
        println!("Line {}, Code :\n{:#}\n", error.code.get_line(), error.code.get_fragment());
    }
}

