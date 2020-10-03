use super::{
    TypeChecker,
    type_check_string,
    get_num_of_errors,
};


/**
 * Test type check let statement.
 */
#[test]
fn test_type_check_let_error() {
    let input1: String = "let test: i32 = false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);
 
    let input2: String = "let test: bool = 20;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check assignment statement.
 */
#[test]
fn test_type_check_assignment_error() {
    let input: String = "let test: mut bool = false; test = 10;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check while statement.
 */
#[test]
fn test_type_check_while_error() {
    let input: String = "fn test() -> () {while 10 {}}".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check if statement.
 */
#[test]
fn test_type_check_if_error() {
    let input: String = "fn test() -> () {if 10 {}}".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check number function parameters.
 */
#[test]
fn test_type_check_num_function_parameters() {
    let input1: String = "fn test(l: bool, r: i32) -> () {test();}".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "fn test() -> () {test(10);}".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check function parameters.
 */
#[test]
fn test_type_check_function_parameters() {
    let input1: String = "fn test(l: bool, r: i32) -> () {test(10, false);}".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 2);

    let input2: String = "fn test(l: bool, r: i32) -> () {test(10, 2);}".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}

/**
 * Test type check function return type.
 */
#[test]
fn test_type_check_return_type() {
    let input1: String = "fn test() -> i32 {return false;}".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "fn test() -> bool {return 0;}".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}

/**
 * Test type check function return is all branches.
 */
#[test]
fn test_type_check_function_return_branches() {
    let input1: String = "fn test() -> i32 {if true {return 2;}}".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "fn test() -> i32 {if false {while false {return 2;}}}".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check undeclered variable.
 */
#[test]
fn test_type_check_undeclered_variable() {
    let input: String = "test = 10;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}

/**
 * Test type check out of scope variable.
 */
#[test]
fn test_type_check_out_of_scope_variable() {
    let input: String = "fn test() -> () {if true {let a: i32 = 0;} a = 10;}".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check undeclered function.
 */
#[test]
fn test_type_check_undeclered_function() {
    let input: String = "test(10);".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}

/**
 * Test type check out of scope function.
 */
#[test]
fn test_type_check_out_of_scope_function() {
    let input: String = "fn test() -> () {if true {fn apa() -> () {}} apa();}".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check unreachable code.
 */
#[test]
fn test_type_check_unreachable_code() {
    let input: String = "fn test() -> i32 {return 2; let a: i32 = 10; return a;}".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 2);
}

/**
 * Test mutability.
 */
#[test]
fn test_mutability_error() {
    let input: String = "let test: bool = false; test = true;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}

