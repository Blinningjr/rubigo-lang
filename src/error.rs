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
}

/**
 * Defines Error in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError {
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
            _ => (),
        };
    }


    /**
     * Print SyntaxError.
     */
    fn print_syntax_error(&mut self, error: SyntaxError) -> () {
        let mut level: String;
        match &error.level {
            ErrorLevel::Critical => level = "Critical Syntax Error".to_string(),
            ErrorLevel::Error => level = "Syntax Error".to_string(),
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

