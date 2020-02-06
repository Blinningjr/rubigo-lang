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
    Operation(),
}


/**
 * Defines function in Rubigo.
 */
pub struct Function {
    ident: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Vec<Expression>,
}


/**
 * Defines while in Rubigo.
 */
pub struct While {
    condition: Expression,
    body: Vec<Expression>,
}


/**
 * Defines if in Rubigo.
 */
pub struct If {
    condition: Expression,
    if_body: Vec<Expression>,
    else_body: Option<Vec<Expression>>,
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
}
