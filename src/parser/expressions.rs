use super::{
    Type,
    Literal,
    Atom,
    Span,
    TypeDecleration,
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
    Assigment(Assigment),
    Return(Return),
    FunctionCall(Box<FunctionCall>),
    Math(Vec<Span<Atom>>), // This is not needed?
    Body(Box<Body>),
    
}


/**
 * Defines Body in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub raw_start: Span<String>,
    pub raw_end: Span<String>,
    pub body: Vec<Expression>,
}


/**
 * Defines function in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub original: Span<String>,
    pub ident: String,
    pub parameters: Vec<Vec<Span<Atom>>>,
    pub return_type: Span<Type>,
    pub body: Body,
}


/** Defines while in Rubigo.  */
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub original: Span<String>, 
    pub condition: Vec<Span<Atom>>,
    pub body: Body,
}


/**
 * Defines if in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub original: Span<String>,
    pub condition: Vec<Span<Atom>>,
    pub if_body: Body,
    pub else_body: Option<Body>,
}


/**
 * Defines let in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub original: Span<String>,
    pub ident: Span<String>,
    pub type_dec: TypeDecleration,
    pub value: Vec<Span<Atom>>,
}


/**
 * Defines Assigment in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Assigment {
    pub original: Span<String>,
    pub ident: Span<String>,
    pub value: Vec<Span<Atom>>,
}


/**
 * Defines return in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub original: Span<String>,
    pub value: Vec<Span<Atom>>,
}


/**
 * Defines function call in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub original: Span<String>,
    pub ident: String,
    pub parameters: Vec<Vec<Span<Atom>>>,
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

