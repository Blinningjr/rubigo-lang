use super::{
    Checker,
    check_string,
    get_num_of_errors,
};


/**
 * Test borrow check double borrow.
 */
#[test]
fn test_borrow_check_double_borrow_error() {
    let input1: String = "let test: &i32 = &(&20);".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 1);
    
    let input2: String = "let mut test: &i32 = &2; test = &(&20);".to_string();
    let checker2: Checker = check_string(input2);
    assert_eq!(get_num_of_errors(checker2), 1);
}

/**
 * Test borrow check deref error.
 */
#[test]
fn test_borrow_check_deref_error() {
    let input1: String = "let test: i32 = *20;".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 1);

    let input2: String = "let mut test: i32 = 20; test = *10;".to_string();
    let checker2: Checker = check_string(input2);
    assert_eq!(get_num_of_errors(checker2), 1);
}

/**
 * Test borrow check mutate borrow value that is not mutable error.
 */
#[test]
fn test_borrow_check_mutate_borrow_error() {
    let input1: String = "let test: &i32 = &20; *test = 10;".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 1);
}

/**
 * Test borrow check borrow value lives shorter then borrower error.
 */
#[test]
fn test_borrow_check_lifetime_scope_error() {
    let input1: String = "let mut test: &i32 = &20; if true {test = &10;}".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 1);
}

