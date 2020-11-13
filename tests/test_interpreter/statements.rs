use super::{
    Value,
    Literal,
    Interpreter,
    Span,
    statement::{
        Statement,
        Function,
        Body,
        Let,
    },
    parser::TypeDecleration,
    interpret_string,

    Expression,
};


/**
 * Test interpret let statement.
 */
#[test]
fn test_interpret_let() {
    let input: String = "let test: mut i32 = 10;".to_string();
    let (values, mut interpreter): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(None));
    assert_eq!(interpreter.get_variable("test".to_string()),
               Value::I32(10));
}


/**
 * Test interpret assignment statement.
 */
#[test]
fn test_interpret_assignment() {
    let input: String = "let test: mut i32 = 10; test = 20;".to_string();
    let (values, mut interpreter): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(None, None));
    assert_eq!(interpreter.get_variable("test".to_string()),
               Value::I32(20));
}


/**
 * Test interpret if (true) statement.
 */
#[test]
fn test_interpret_if() {
    let input: String = "if true {return 10;} else {return 20;}".to_string();
    let (values, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(Some(Value::I32(10))));
}

/**
 * Test interpret else (if false) statement.
 */
#[test]
fn test_interpret_else() {
    let input: String = "if false {return 10;} else {return 20;}".to_string();
    let (values, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(Some(Value::I32(20))));
}


/**
 * Test interpret while statement.
 */
#[test]
fn test_interpret_while() {
    let input: String = "let i: mut i32 = 0; while i < 10 {i = i + 1;}".to_string();
    let (values, mut interpreter): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(None, None));
    assert_eq!(interpreter.get_variable("i".to_string()),
               Value::I32(10));
}


/**
 * Test interpret return statement.
 */
#[test]
fn test_interpret_return() {
    let input: String = "return 10;".to_string();
    let (values, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(Some(Value::I32(10))));
}


/**
 * Test interpret body statement.
 */
#[test]
fn test_interpret_body() {
    let input: String = "{let test: i32 = 10; return test;}".to_string();
    let (values, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(Some(Value::I32(10))));
}


/**
 * Test interpret function statement.
 */
#[test]
fn test_interpret_function() {
    let input: String = "fn test() -> i32 {}".to_string();
    let (_, mut interpreter): (Vec<Option<Value>>, Interpreter) = interpret_string(input.clone());
    assert_eq!(interpreter.get_function("test".to_string()),
               Function{
                    original: Span::new(input, 1, 1),
                    id: 1,
                    identifier: Span::new("test".to_string(), 1, 4),
                    parameters: vec!(),
                    body: Body{
                        original: Span::new(" {}".to_string(), 1, 17),
                        id: 0,
                        body: vec!(),
                    },
                    return_type: TypeDecleration{
                        borrow: false,
                        mutable: false,
                        r#type: Span::new("i32".to_string(), 1, 14),                        
                    },
               });
}


/**
 * Test interpret function call statement.
 */
#[test]
fn test_interpret_function_call() {
    let input: String = "fn duble(num: i32) -> i32 {return num * 2;} return duble(10);".to_string();
    let (values, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input);
    assert_eq!(values, vec!(None, Some(Value::I32(20))));
}

