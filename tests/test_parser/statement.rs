use super::{
    parse_string,
    span::Span,
};

use super::parser::statement::{
    Statement,
    Let,
    Assignment,
    If,
    While,
    Body,
    Function,
    Return,
};

use super::parser::expressions::{
    Expression,
    FunctionCall,
};

use super::parser::type_decleration::TypeDecleration;
use super::parser::literal::Literal;


/**
 * Test parse let statement.
 */
#[test]
fn test_parse_let() {
    let input: String = "let test: i32 = 10;".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Let(Let {
            id: 0,
            original: Span::new(input.clone(), 1, 1),
            identifier: Span::new("test".to_string(), 1, 5),
            type_dec: TypeDecleration {
                borrow: false,
                mutable: false,
                r#type: Span::new("i32".to_string(), 1, 11),
            },
            value: Expression::Literal(Literal::I32(
                           Span::new(10, 1, 17))), 
        })
    );
}


/**
 * Test parse assignment statement.
 */
#[test]
fn test_parse_assignment() {
    let input: String = "test = 10;".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Assignment(Assignment {
            id: 0,
            original: Span::new(input.clone(), 1, 1),
            identifier: Span::new("test".to_string(), 1, 1),
            value: Expression::Literal(Literal::I32(Span::new(10, 1, 8))), 
        })
    );
}


/**
 * Test parse function call statement.
 */
#[test]
fn test_parse_function_call() {
    let input: String = "test(10);".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Expression(Expression::FunctionCall(Box::new(FunctionCall {
            id: 0,
            original: Span::new("test(10)".to_string(), 1, 1),
            identifier: Span::new("test".to_string(), 1, 1),
            parameters: vec!(Expression::Literal(Literal::I32(Span::new(10, 1, 6)))), 
        })))
    );
}


/**
 * Test parse if statement.
 */
#[test]
fn test_parse_if() {
    let input: String = "if true {\ntest = 10;\n}".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::If(Box::new(If {
            id: 0,
            original: Span::new(input.clone(), 1, 1),
            condition: Expression::Literal(Literal::Bool(Span::new(true, 1, 4))),
            if_body: Body {
                id: 0,
                original: Span::new(" {\ntest = 10;\n}".to_string(), 1, 8),
                body: vec!(Statement::Assignment(Assignment {
                    id: 1,
                    original: Span::new("test = 10;".to_string(), 2, 1),
                    identifier: Span::new("test".to_string(), 2, 1),
                    value: Expression::Literal(Literal::I32(Span::new(10, 2, 8))), 
                })),
            },
            else_body: Option::None, 
        }))
    );
}


/**
 * Test parse if else statement.
 */
#[test]
fn test_parse_if_else() {
    let input: String = "if true {\ntest = 10;\n} else {}".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::If(Box::new(If {
            id: 0,
            original: Span::new(input.clone(), 1, 1),
            condition: Expression::Literal(Literal::Bool(Span::new(true, 1, 4))),
            if_body: Body {
                id: 0,
                original: Span::new(" {\ntest = 10;\n}".to_string(), 1, 8),
                body: vec!(Statement::Assignment(Assignment {
                    id: 1,
                    original: Span::new("test = 10;".to_string(), 2, 1),
                    identifier: Span::new("test".to_string(), 2, 1),
                    value: Expression::Literal(Literal::I32(Span::new(10, 2, 8))), 
                })),
            },
            else_body: Option::Some(Body {
                id: 0,
                original: Span::new(" {}".to_string(), 3, 7),
                body: vec!(),
            }), 
        }))
    );
}


/**
 * Test parse while statement.
 */
#[test]
fn test_parse_while() {
    let input: String = "while true {\ntest = 10;\n}".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::While(Box::new(While {
            id: 0,
            original: Span::new(input.clone(), 1, 1),
            condition: Expression::Literal(Literal::Bool(Span::new(true, 1, 7))),
            body: Body {
                id: 0,
                original: Span::new(" {\ntest = 10;\n}".to_string(), 1, 11),
                body: vec!(Statement::Assignment(Assignment {
                    id: 1,
                    original: Span::new("test = 10;".to_string(), 2, 1),
                    identifier: Span::new("test".to_string(), 2, 1),
                    value: Expression::Literal(Literal::I32(Span::new(10, 2, 8))), 
                })),
            },
        }))
    );
}


/**
 * Test parse body statement.
 */
#[test]
fn test_parse_body() {
    let input: String = "{\ntest = 10;\n}".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Body(Box::new(Body {
            id: 0,
            original: Span::new("{\ntest = 10;\n}".to_string(), 1, 1),
            body: vec!(Statement::Assignment(Assignment {
                id: 1,
                original: Span::new("test = 10;".to_string(), 2, 1),
                identifier: Span::new("test".to_string(), 2, 1),
                value: Expression::Literal(Literal::I32(Span::new(10, 2, 8))), 
            })),    
        }))
    );
}


/**
 * Test parse function statement.
 */
#[test]
fn test_parse_function() {
    let input: String = "fn test(t: i32) -> i32 {\ntest = 10;\n}".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Function(Box::new(Function {
            id: 1,
            original: Span::new(input.clone(), 1, 1),
            identifier: Span::new("test".to_string(), 1, 4),
            parameters: vec!((Span::new("t".to_string(), 1, 9),
                TypeDecleration{
                    borrow: false,
                    mutable: false,
                    r#type: Span::new("i32".to_string(), 1, 12),
                }
            )),
            return_type: TypeDecleration{
                borrow: false,
                mutable: false,
                r#type: Span::new("i32".to_string(), 1, 20),
            },
            body: Body {
                id: 0,
                original: Span::new(" {\ntest = 10;\n}".to_string(), 1, 23),
                body: vec!(Statement::Assignment(Assignment {
                    id: 0,
                    original: Span::new("test = 10;".to_string(), 2, 1),
                    identifier: Span::new("test".to_string(), 2, 1),
                    value: Expression::Literal(Literal::I32(Span::new(10, 2, 8))), 
                })),
            },
        }))
    );
}


/**
 * Test parse return statement.
 */
#[test]
fn test_parse_return() {
    let input: String = "return 10;".to_string();
    let statement: Statement = parse_string(input.clone());
    assert_eq!(statement, 
        Statement::Return(Return {
            id: 0,
            original: Span::new("return 10;".to_string(), 1, 1),
            value: Expression::Literal(Literal::I32(Span::new(10, 1, 8))),
        })
    );
}

