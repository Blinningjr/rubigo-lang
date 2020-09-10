use super::{
    TypeChecker,
    type_check_string,
    check_no_errors,
    get_num_of_errors,
};


/**
 * Test type check unary operator not.
 */
#[test]
fn test_type_check_unop_not() {
    let input: String = "let test: bool = !false;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert!(check_no_errors(type_checker));
}


/**
 * Test type check unary operator not.
 */
#[test]
fn test_type_check_unop_not_error() {
    let input: String = "let test: bool = !10;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check unary operator minus.
 */
#[test]
fn test_type_check_unop_minus() {
    let input1: String = "let test: i32 = -10;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));

    let input2: String = "let test: i32 = -10.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check unary operator minus.
 */
#[test]
fn test_type_check_unop_minus_error() {
    let input: String = "let test: i32 = -false;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert_eq!(get_num_of_errors(type_checker), 1);
}


/**
 * Test type check binary operator plus.
 */
#[test]
fn test_type_check_binop_plus() {
    let input1: String = "let test: i32 = 10 + 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: f32 = 10.3 + 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator plus.
 */
#[test]
fn test_type_check_binop_plus_error() {
    let input1: String = "let test: i32  = 10 + false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: f32  = false + 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator minus.
 */
#[test]
fn test_type_check_binup_minus() {
    let input1: String = "let test: i32 = 10 - 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: f32 = 10.3 - 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator minus.
 */
#[test]
fn test_type_check_binop_minus_error() {
    let input1: String = "let test: i32  = 10 - false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: f32  = false - 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator divition.
 */
#[test]
fn test_type_check_binop_divition() {
    let input1: String = "let test: i32 = 10 / 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: f32 = 10.3 / 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator divition.
 */
#[test]
fn test_type_check_binop_divition_error() {
    let input1: String = "let test: i32  = 10 / false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: f32  = false / 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator multiplication.
 */
#[test]
fn test_type_check_binop_multiplication() {
    let input1: String = "let test: i32 = 10 * 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: f32 = 10.3 * 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator multiplication.
 */
#[test]
fn test_type_check_binop_multiplication_error() {
    let input1: String = "let test: i32  = 10 * false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: f32  = false * 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator modilus.
 */
#[test]
fn test_type_check_binop_modilus() {
    let input1: String = "let test: i32 = 10 % 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: f32 = 10.3 % 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator modilus.
 */
#[test]
fn test_type_check_binop_modilus_error() {
    let input1: String = "let test: i32  = 10 % false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: f32  = false % 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator less then.
 */
#[test]
fn test_type_check_binop_lessthen() {
    let input1: String = "let test: bool = 10 < 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 < 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator less then.
 */
#[test]
fn test_type_check_binop_lessthen_error() {
    let input1: String = "let test: bool  = 10 < false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false < 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator greater then.
 */
#[test]
fn test_type_check_binop_greaterthen() {
    let input1: String = "let test: bool = 10 > 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 > 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator greater then.
 */
#[test]
fn test_type_check_binop_greaterthen_error() {
    let input1: String = "let test: bool  = 10 > false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false > 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator not equal.
 */
#[test]
fn test_type_check_binop_notequal() {
    let input1: String = "let test: bool = 10 != 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 != 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator not equal.
 */
#[test]
fn test_type_check_binop_notequal_error() {
    let input1: String = "let test: bool  = 10 != false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false != 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator equal.
 */
#[test]
fn test_type_check_binop_equal() {
    let input1: String = "let test: bool = 10 == 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 == 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator equal.
 */
#[test]
fn test_type_check_binop_equal_error() {
    let input1: String = "let test: bool  = 10 == false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false == 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator greater equal.
 */
#[test]
fn test_type_check_binop_greaterequal() {
    let input1: String = "let test: bool = 10 >= 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 >= 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator greater equal.
 */
#[test]
fn test_type_check_binop_greaterequal_error() {
    let input1: String = "let test: bool  = 10 >= false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false >= 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator less equal.
 */
#[test]
fn test_type_check_binop_lessequal() {
    let input1: String = "let test: bool = 10 <= 12;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert!(check_no_errors(type_checker1));
    
    let input2: String = "let test: bool = 10.3 <= 12.6;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert!(check_no_errors(type_checker2));
}


/**
 * Test type check binary operator less equal.
 */
#[test]
fn test_type_check_binop_lessequal_error() {
    let input1: String = "let test: bool  = 10 <= false;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false <= 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator and.
 */
#[test]
fn test_type_check_binop_and() {
    let input: String = "let test: bool = false && true;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert!(check_no_errors(type_checker));
}


/**
 * Test type check binary operator and.
 */
#[test]
fn test_type_check_binop_and_error() {
    let input1: String = "let test: bool  = 10 && 10;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false && 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}


/**
 * Test type check binary operator or.
 */
#[test]
fn test_type_check_binop_or() {
    let input: String = "let test: bool = false || true;".to_string();
    let type_checker: TypeChecker = type_check_string(input);
    assert!(check_no_errors(type_checker));
}


/**
 * Test type check binary operator or.
 */
#[test]
fn test_type_check_binop_or_error() {
    let input1: String = "let test: bool  = 10 || 10;".to_string();
    let type_checker1: TypeChecker = type_check_string(input1);
    assert_eq!(get_num_of_errors(type_checker1), 1);

    let input2: String = "let test: bool  = false || 20.2;".to_string();
    let type_checker2: TypeChecker = type_check_string(input2);
    assert_eq!(get_num_of_errors(type_checker2), 1);
}

