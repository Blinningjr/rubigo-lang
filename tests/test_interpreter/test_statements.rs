use super::{
    interpret_modual,
    Literal,
    Interpreter,
    InterpEnv,
    Span,
};


/**
 * Test interpret let statement.
 */
#[test]
fn test_interpret_let() {
    let input: String = "let test: i32 = 10;".to_string();
    let (literal, mut interpreter): (Literal, Interpreter) = interpret_modual(input);
    assert_eq!(literal, Literal::Dummy);

    assert_eq!(interpreter.get_variable("test".to_string()),
               Literal::I32(Span::new(10, 1, 17)));
}


/**
 * Test interpret assignment statement.
 */
#[test]
fn test_interpret_assignment() {
    let input: String = "let test: mut i32 = 10; test = 20;".to_string();
    let (literal, mut interpreter): (Literal, Interpreter) = interpret_modual(input);
    assert_eq!(literal, Literal::Dummy);

    assert_eq!(interpreter.get_variable("test".to_string()),
               Literal::I32(Span::new(20, 1, 32)));
}


/**
 * Test interpret if statement.
 */
#[test]
fn test_interpret_if() {
    let input: String = "if true {return 10;} else {return 20;}".to_string();
    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
    assert_eq!(literal,
               Literal::I32(Span::new(10, 1, 17)));
}


/**
 * Test interpret else statement.
 */
#[test]
fn test_interpret_else() {
    let input: String = "if false {return 10;} else {return 20;}".to_string();
    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
    assert_eq!(literal,
               Literal::I32(Span::new(20, 1, 36)));
}


/**
 * Test interpret while statement.
 */
#[test]
fn test_interpret_while() {
    let input: String = "let i: mut i32 = 0; while i < 10 {i = i + 1;} return i;".to_string();
    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
    assert_eq!(literal,
               Literal::I32(Span::new(10, 1, 18)));
}

