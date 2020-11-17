use super::{
    Checker,
    check_string,
    check_no_errors,
};


/**
 * Test type check let statement.
 */
#[test]
fn test_type_check_let() {
    let input: String = "let test: i32 = 10;".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}


/**
 * Test type check assignment statement.
 */
#[test]
fn test_type_check_assignment() {
    let input: String = "let mut test: i32 = 10; test = 2;".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}


/**
 * Test type check while statement codition.
 */
#[test]
fn test_type_check_while_condition() {
    let input: String = "fn test() -> () {while false {}}".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}


/**
 * Test type check if statement codition.
 */
#[test]
fn test_type_check_if_condition() {
    let input: String = "fn test() -> () {if false {}}".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}


/**
 * Test type check function call.
 */
#[test]
fn test_type_check_function() {
    let input: String = "fn test(l: i32, b: bool) -> () {test(1, false);}".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}


/**
 * Test type check return statement.
 */
#[test]
fn test_type_check_return() {
    let input: String = "fn test(l: i32, b: bool) -> bool {return true;}".to_string();
    let checker: Checker = check_string(input);
    assert!(check_no_errors(checker));
}

