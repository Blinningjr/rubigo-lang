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
    interpret_statement,

    Expression,
};


/**
 * Test interpret let statement.
 */
#[test]
fn test_interpret_let() {
    let stmt: Statement = Statement::Let(Let{
        id: 0,
        original: Span::new("".to_string(), 1, 1),
        mutable: None,
        identifier: Span::new("test".to_string(), 1, 1),
        type_dec: TypeDecleration{
            borrow: false,
            mutable: false,
            r#type: Span::new("i32".to_string(), 1, 1),
        },
        value: Expression::Literal(Literal::I32(Span::new(10, 1, 17))),
    });
    let (value, mut interpreter): (Option<Value>, Interpreter) = interpret_statement(stmt);
    assert_eq!(value, None);

    assert_eq!(interpreter.get_variable("test".to_string()),
               Value::I32(10));
}


//#[test]
//fn test_interpret_assignment() {
//    let input: String = "let test: mut i32 = 10; test = 20;".to_string();
//    let (literal, mut interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal, Literal::Dummy);
//
//    assert_eq!(interpreter.get_variable("test".to_string()),
//               Literal::I32(Span::new(20, 1, 32)));
//}


//#[test]
//fn test_interpret_if() {
//    let input: String = "if true {return 10;} else {return 20;}".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(10, 1, 17)));
//}

//#[test]
//fn test_interpret_else() {
//    let input: String = "if false {return 10;} else {return 20;}".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(20, 1, 36)));
//}


//#[test]
//fn test_interpret_while() {
//    let input: String = "let i: mut i32 = 0; while i < 10 {i = i + 1;} return i;".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(10, 1, 18)));
//}


//#[test]
//fn test_interpret_return() {
//    let input: String = "return 10;".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(10, 1, 8)));
//}


//#[test]
//fn test_interpret_body() {
//    let input: String = "{return 10;}".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(10, 1, 9)));
//}


//#[test]
//fn test_interpret_function() {
//    let input: String = "fn test() -> i32 {}".to_string();
//    let (_literal, mut interpreter): (Literal, Interpreter) = interpret_modual(input.clone());
//    assert_eq!(interpreter.get_function("test".to_string(), 0),
//               Function{
//                    original: Span::new(input, 1, 1),
//                    id: 1,
//                    identifier: Span::new("test".to_string(), 1, 4),
//                    parameters: vec!(),
//                    body: Body{
//                        original: Span::new(" {}".to_string(), 1, 17),
//                        id: 0,
//                        body: vec!(),
//                    },
//                    return_type: TypeDecleration{
//                        borrow: false,
//                        mutable: false,
//                        r#type: Span::new("i32".to_string(), 1, 14),                        
//                    },
//               });
//}


//#[test]
//fn test_interpret_function_call() {
//    let input: String = "fn duble(num: i32) -> i32 {return num * 2;} return duble(10);".to_string();
//    let (literal, _interpreter): (Literal, Interpreter) = interpret_modual(input);
//    assert_eq!(literal,
//               Literal::I32(Span::new(20, 1, 58)));
//}

