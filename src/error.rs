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
pub struct Error {
    pub level: ErrorLevel,
    pub message: String,
//  pub code: Sting, TODO:Implement so the code is displayed in the error.
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
     * Creates a new ErrorHandler.
     */
    pub fn new(verbose: bool) -> ErrorHandler {
        return ErrorHandler {
            errors: Vec::new(),
            verbose: verbose,
        };
    }


    /**
     * Add Error..
     */
    pub fn add(&mut self, error: Error) -> () {
        self.errors.push(error.clone());
        
        if error.level == ErrorLevel::Critical {
            self.print_errors();
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
        let mut level: String;
        match &error.level {
            ErrorLevel::Critical => level = "Critical Error".to_string(),
            ErrorLevel::Error => level = "Error".to_string(),
            ErrorLevel::Warning => level = "Warning".to_string(),
        };

        let mut location: String = format!(" at {:?}:{:?}", error.line, error.offset).to_string();
        if error.line == 0 && error.offset == 0 {
            location =  "".to_string();
        }
        println!("{}{}", level, location);
        println!("\t{:?}\n", error.message);
    }
}

