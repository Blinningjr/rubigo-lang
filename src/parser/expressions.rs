use super::{
    Type,
    Literal,
    Atom,
    Span,
};


/**
 * Defines all expressions in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Function(Box<Function>),
    While(Box<While>),
    If(Box<If>),
    Let(Let),
    Return(Vec<Span<Atom>>),
    FunctionCall(Box<FunctionCall>),
    Math(Vec<Span<Atom>>),
    Body(Box<Body>),
    
}


/**
 * Defines Body in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    body: Vec<Expression>,
}


/**
 * Defines function in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    ident: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Body,
}


/** Defines while in Rubigo.  */
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    condition: Expression,
    body: Body,
}


/**
 * Defines if in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    condition: Expression,
    if_body: Body,
    else_body: Body,
}


/**
 * Defines let in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub original: Span<String>,
    pub ident: Span<String>,
    pub r#type: Span<Type>,
    pub value: Vec<Span<Atom>>,
}


/**
 * Defines return in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
   r#type: Type,
   valuse: Expression,
}


/**
 * Defines function call in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    ident: String,
    parameters: Vec<Parameter>,
}


/**
 * Defines parameter in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    ident: String,
    r#type: Type,
    line: usize,
    start_col: usize,
    end_col: usize,
}

