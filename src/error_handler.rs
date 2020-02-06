use std::Display;


/**
 * Keeps track of all errors.
 */
pub struct ErrorHandler<T: Display> {
    errors: Vec<T>
}


impl ErrorHandler {
    pub fn new() -> ErrorHandler {
        return ErrorHandler {
            errors: Vec::new(),
        };
    }
}


impl ErrorHandler {
    pub fn append<T: Display>(&mut self, error: T>) {
        self.errors.append(error);
    }
}


impl ErrorHandler {
    pub fn print_errors<T: Display>(&self) {
        for error in self.errors {
            print!("{}", error.to_string());
        }
    }
}
