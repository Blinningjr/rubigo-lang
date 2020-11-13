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
 * Test interpret unop not operation.
 */
#[test]
fn test_interpret_unop_not() {
    let input1: String = "return !false;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(true))));

    let input2: String = "return !true;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(false))));
}


/**
 * Test interpret unop minus operation.
 */
#[test]
fn test_interpret_unop_minus() {
    let input1: String = "return -10;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(-10))));

    let input2: String = "return -(-10);".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::I32(10))));
}


/**
 * Test interpret binop plus operation.
 */
#[test]
fn test_interpret_binop_plus() {
    let input1: String = "return 5+2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(7))));
}


/**
 * Test interpret binop minus operation.
 */
#[test]
fn test_interpret_binop_minus() {
    let input1: String = "return 5-2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(3))));

    let input2: String = "return 2-5;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::I32(-3))));
}


/**
 * Test interpret binop divition operation.
 */
#[test]
fn test_interpret_binop_divition() {
    let input1: String = "return 6/2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(3))));

    let input2: String = "return 5/2;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::I32(2))));
}


/**
 * Test interpret binop multiplication operation.
 */
#[test]
fn test_interpret_binop_multiplication() {
    let input1: String = "return 5*2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(10))));

    let input2: String = "return 5*(-2);".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::I32(-10))));
}


/**
 * Test interpret binop modilus operation.
 */
#[test]
fn test_interpret_binop_modilus() {
    let input1: String = "return 5%2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::I32(1))));

    let input2: String = "return 4%2;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::I32(0))));
}


/**
 * Test interpret binop lessthen operation.
 */
#[test]
fn test_interpret_binop_lessthen() {
    let input1: String = "return 5<2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(false))));

    let input2: String = "return 2<2;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(false))));

    let input3: String = "return 2<5;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(true))));
}


/**
 * Test interpret binop greaterthen operation.
 */
#[test]
fn test_interpret_binop_greaterthen() {
    let input1: String = "return 5>2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(true))));

    let input2: String = "return 2>2;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(false))));

    let input3: String = "return 2>5;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(false))));
}


/**
 * Test interpret binop notequal operation.
 */
#[test]
fn test_interpret_binop_notequal() {
    let input1: String = "return 5 != 2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(true))));

    let input2: String = "return 5 != 5;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(false))));
}


/**
 * Test interpret binop equal operation.
 */
#[test]
fn test_interpret_binop_equal() {
    let input1: String = "return 5==2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(false))));

    let input2: String = "return 2==2;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(true))));
}


/**
 * Test interpret binop greaterequal operation.
 */
#[test]
fn test_interpret_binop_greaterequal() {
    let input1: String = "return 5>=2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(true))));

    let input2: String = "return 5>=5;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(true))));

    let input3: String = "return 2>=5;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(false))));
}


/**
 * Test interpret binop lessequal operation.
 */
#[test]
fn test_interpret_binop_lessequal() {
    let input1: String = "return 5<=2;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(false))));

    let input2: String = "return 5<=5;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(true))));

    let input3: String = "return 2<=5;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(true))));
}


/**
 * Test interpret binop and operation.
 */
#[test]
fn test_interpret_binop_and() {
    let input1: String = "return true && false;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(false))));

    let input2: String = "return true && true;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(true))));

    let input3: String = "return false && false;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(false))));
}


/**
 * Test interpret binop or operation.
 */
#[test]
fn test_interpret_binop_or() {
    let input1: String = "return true || false;".to_string();
    let (values1, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input1);
    assert_eq!(values1, vec!(Some(Value::Bool(true))));

    let input2: String = "return true || true;".to_string();
    let (values2, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input2);
    assert_eq!(values2, vec!(Some(Value::Bool(true))));

    let input3: String = "return false || false;".to_string();
    let (values3, _): (Vec<Option<Value>>, Interpreter) = interpret_string(input3);
    assert_eq!(values3, vec!(Some(Value::Bool(false))));
}

