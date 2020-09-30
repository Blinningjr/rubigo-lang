use super::{
    interpret_a_statement,
    Literal,
};


/**
 * Test type check let statement.
 */
#[test]
fn test_interpret_let() {
    let input: String = "let test: i32 = 10;".to_string();
    let literal: Literal = interpret_a_statement(input);
    assert_eq!(literal, Literal::Dummy);
}

