// /**
//  * Defines the traits all errors must have.
//  */
// pub trait Error {
//     fn to_string(&self);
// }


// /**
//  * Keeps track of all errors.
//  */
// pub struct ErrorHandler<T: Error> {
//     errors: Vec<T>
// }


// /**
//  * Creates a new error handler.
//  */
// impl<T: Error> ErrorHandler<T> {
//     pub fn new() -> ErrorHandler<T> {
//         return ErrorHandler::<T> {
//             errors: Vec::new(),
//         };
//     }
// }


// // /**
// //  * Appends a error to the errors vector.
// //  */
// // impl<T: Error + Clone> ErrorHandler<T> {
// //     pub fn append(&mut self, error: T) {
// //         self.errors.append(error);
// //     }
// // }


// /**
//  * Prints all errors stored in error handler.
//  */
// impl<T: Error> ErrorHandler<T> {
//     pub fn print_errors(&self) {
//         for error in self.errors {
//             print!("{:#?}", error.to_string());
//         }
//     }
// }
