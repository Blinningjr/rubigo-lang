use super::{
    Checker,
    check_string,
    get_num_of_errors,
};


/**
 * Test borrow check borrow.
 */
#[test]
fn test_borrow_check_borrow() {
    let input1: String = "let test: &i32 = &20;".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 0);
    
    let input2: String = "let test: i32 = 2; let test2: &i32 = &test;".to_string();
    let checker2: Checker = check_string(input2);
    assert_eq!(get_num_of_errors(checker2), 0);
}

/**
 * Test borrow check deref.
 */
#[test]
fn test_borrow_check_deref() {
    let input1: String = "let test: &i32 = &20; let test2: i32 = *test;".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 0);
}

/**
 * Test borrow check mutable borrow.
 */
#[test]
fn test_borrow_check_mutate_borrow() {
    let input1: String = "let mut test: i32 = 20; let test2: &mut i32 = &mut test; *test2 = 10;".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 0);
}

/**
 * Test borrow check borrow value lives shorter then borrower error.
 */
#[test]
fn test_borrow_check_lifetime_scope() {
    let input1: String = "let test: i32 = 20; if true {let test2: &i32 = &test;}".to_string();
    let checker1: Checker = check_string(input1);
    assert_eq!(get_num_of_errors(checker1), 0);
}

