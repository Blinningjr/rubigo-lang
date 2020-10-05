#![allow(dead_code)]


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Number(bool, bool),
    Custom(String, bool, bool),
}

impl Type {
    pub fn to_string(& self) -> String {
        return match self {
            Type::Any => "<ANY>".to_string(),
            Type::Number(borrowed, muta) => {
                let borrow = match borrowed { true => "&", false => "",};
                let mutable = match muta { true => "mut", false => "",};
                return format!("{}{} <A Number>", borrow, mutable);
            },
            Type::Custom(string, borrowed, muta) => {
                let borrow = match borrowed { true => "&", false => "",};
                let mutable = match muta { true => "mut", false => "",};
                return format!("{}{} <A Number>", borrow, mutable);
            },
        };
    }
}

pub fn compare_types(type1: &Type, type2: &Type) -> bool {
    match type1 {
        Type::Any => return true,
        Type::Number(t1_b, t1_m) => {
            match type2 {
                Type::Any => return true,
                Type::Number(t2_b, t2_m) => return t1_b == t2_b && t1_m == t2_m,
                Type::Custom(_type2_string, t2_b, t2_m) => return is_type_number(type2) && t1_b == t2_b && t1_m == t2_m,
            };
        },
        Type::Custom(type1_string, t1_b, t1_m) => {
            match type2 {
                Type::Any => return true,
                Type::Number(t2_b, t2_m) => return is_type_number(type1) && t1_b == t2_b && t1_m == t2_m,
                Type::Custom(type2_string, t2_b, t2_m) => return type1_string == type2_string && t1_b == t2_b && t1_m == t2_m,
            };
        },
    };
}

pub fn is_type_number(r#type: &Type) -> bool {
    match r#type {
        Type::Any => return true,
        Type::Number(_, _) => return true,
        Type::Custom(type_string, _, _) => return type_string == "i32" || type_string == "f32",
    };
}

