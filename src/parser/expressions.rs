use super::{
    Type,
    Literal,
};


/**
 * Defines all expressions in Rubigo.
 */
pub enum Expression {
    Function(Box<Function>),
    While(Box<While>),
    If(Box<If>),
    Let(Box<Let>),
    Return(Return),
    FunctionCall(Box<FunctionCall>),
    Math(),
    Body(Box<Body>)
}


/**
 * Defines Body in Rubigo.
 */
pub struct Body {
    Body: Vec<Expression>,
}


/**
 * Defines function in Rubigo.
 */
pub struct Function {
    ident: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Body,
}


/** Defines while in Rubigo.  */
pub struct While {
    condition: Expression,
    body: Body,
}


/**
 * Defines if in Rubigo.
 */
pub struct If {
    condition: Expression,
    if_body: Body,
    else_body: Body,
}


/**
 * Defines let in Rubigo.
 */
pub struct Let {
    ident: String,
    r#type: Type,
    parameters: Vec<Parameter>,
}


/**
 * Defines return in Rubigo.
 */
pub struct Return {
   r#type: Type,
   valuse: Expression,
}


/**
 * Defines function call in Rubigo.
 */
pub struct FunctionCall {
    ident: String,
    parameters: Vec<Parameter>,
}


/**
 * Defines parameter in Rubigo.
 */
pub struct Parameter {
    ident: String,
    r#type: Type,
    line: usize,
    start_col: usize,
    end_col: usize,
}


/**
 * Defines parameter in Rubigo.
 */
pub struct Math {
    ident: String,
    r#type: Type,
}
