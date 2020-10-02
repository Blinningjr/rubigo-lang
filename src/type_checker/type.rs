#![allow(dead_code)]


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Number,
    Custom(String),
}

impl Type {
    pub fn to_string(& self) -> String {
        return match self {
            Type::Any => "<ANY>".to_string(),
            Type::Number => "<A Number>".to_string(),
            Type::Custom(string) => string.clone(),
        };
    }
}

pub fn compare_types(type1: &Type, type2: &Type) -> bool {
    match type1 {
        Type::Any => return true,
        Type::Number => return is_type_number(type2),
        Type::Custom(type1_string) => {
            match type2 {
                Type::Any => return true,
                Type::Number => return is_type_number(type1),
                Type::Custom(type2_string) => return type1_string == type2_string,
            };

        },
    };
}

pub fn is_type_number(r#type: &Type) -> bool {
    match r#type {
        Type::Any => return true,
        Type::Number => return true,
        Type::Custom(type_string) => return type_string == "i32" || type_string == "f32",
    };
}

