/**
 * All types defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    F32,
    Bool,
    Char,
    String,
}


/**
 * All literals defined in Rubigo
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I32(i32),
    F32(f32),
    Bool(bool),
    Char(char),
    String(String),
}
