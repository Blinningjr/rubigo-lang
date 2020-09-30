use super::{
    interpret_modual,
    Literal,
    Interpreter,
    Span,
};


/**
 * Test interpret not unary operation.
 */
#[test]
fn test_interpret_unop_not() {
    let input1: String = "return !false;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(true, 0, 0)));
    
    let input2: String = "return !true;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(false, 0, 0)));
}


/**
 * Test interpret not unary operation.
 */
#[test]
fn test_interpret_unop_minus() {
    let input1: String = "return -10;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(-10, 1, 9)));

    let input2: String = "return -(-10);".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::I32(Span::new(10, 1, 11)));
}


/**
 * Test interpret plus binary operation.
 */
#[test]
fn test_interpret_binop_plus() {
    let input1: String = "return 5+2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(7, 1, 8)));
}


/**
 * Test interpret minus binary operation.
 */
#[test]
fn test_interpret_binop_minus() {
    let input1: String = "return 5-2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(3, 1, 8)));
    
    let input2: String = "return 2-5;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::I32(Span::new(-3, 1, 8)));
}


/**
 * Test interpret divition binary operation.
 */
#[test]
fn test_interpret_binop_divition() {
    let input1: String = "return 6/2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(3, 1, 8)));

    let input2: String = "return 5/2;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::I32(Span::new(2, 1, 8)));
}


/**
 * Test interpret multiplication binary operation.
 */
#[test]
fn test_interpret_binop_multiplication() {
    let input1: String = "return 5*2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(10, 1, 8)));

    let input2: String = "return 5*(-2);".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::I32(Span::new(-10, 1, 8)));
}


/**
 * Test interpret modilus binary operation.
 */
#[test]
fn test_interpret_binop_modilus() {
    let input1: String = "return 5%2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::I32(Span::new(1, 1, 8)));

    let input2: String = "return 4%2;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::I32(Span::new(0, 1, 8)));
}


/**
 * Test interpret lessthen binary operation.
 */
#[test]
fn test_interpret_binop_lessthen() {
    let input1: String = "return 5<2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(false, 1, 8)));

    let input2: String = "return 2<2;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(false, 1, 8)));

    let input3: String = "return 2<5;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(true, 1, 8)));
}


/**
 * Test interpret graterthen binary operation.
 */
#[test]
fn test_interpret_binop_graterthen() {
    let input1: String = "return 5>2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(true, 1, 8)));

    let input2: String = "return 2>2;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(false, 1, 8)));

    let input3: String = "return 2>5;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(false, 1, 8)));
}


/**
 * Test interpret notequal binary operation.
 */
#[test]
fn test_interpret_binop_notequal() {
    let input1: String = "return 5 != 2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(true, 1, 8)));

    let input2: String = "return 5 != 5;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(false, 1, 8)));
}


/**
 * Test interpret equal binary operation.
 */
#[test]
fn test_interpret_binop_equal() {
    let input1: String = "return 5==2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(false, 1, 8)));

    let input2: String = "return 2==2;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(true, 1, 8)));
}


/**
 * Test interpret greaterequal binary operation.
 */
#[test]
fn test_interpret_binop_greaterequal() {
    let input1: String = "return 5>=2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(true, 1, 8)));

    let input2: String = "return 5>=5;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(true, 1, 8)));

    let input3: String = "return 2>=5;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(false, 1, 8)));
}


/**
 * Test interpret lessequal binary operation.
 */
#[test]
fn test_interpret_binop_lessequal() {
    let input1: String = "return 5<=2;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(false, 1, 8)));

    let input2: String = "return 5<=5;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(true, 1, 8)));

    let input3: String = "return 2<=5;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(true, 1, 8)));
}


/**
 * Test interpret and binary operation.
 */
#[test]
fn test_interpret_binop_and() {
    let input1: String = "return true && false;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(false, 0, 0)));

    let input2: String = "return true && true;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(true, 0, 0)));

    let input3: String = "return false && false;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(false, 0, 0)));
}


/**
 * Test interpret or binary operation.
 */
#[test]
fn test_interpret_binop_or() {
    let input1: String = "return true || false;".to_string();
    let (literal1, _interpreter1): (Literal, Interpreter) = interpret_modual(input1);
    assert_eq!(literal1, 
               Literal::Bool(Span::new(true, 0, 0)));

    let input2: String = "return true || true;".to_string();
    let (literal2, _interpreter2): (Literal, Interpreter) = interpret_modual(input2);
    assert_eq!(literal2, 
               Literal::Bool(Span::new(true, 0, 0)));

    let input3: String = "return false || false;".to_string();
    let (literal3, _interpreter3): (Literal, Interpreter) = interpret_modual(input3);
    assert_eq!(literal3, 
               Literal::Bool(Span::new(false, 0, 0)));
}

