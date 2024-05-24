pub mod builtin;
pub mod function;

use std::fmt::Display;

use self::{builtin::BuiltinFunction, function::Function};

#[derive(PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Char(char),
    Str(String),
    Boolean(bool),
    Function(Function),
    Builtin(BuiltinFunction),
    Array(Vec<Object>),
    Return(Box<Object>),
    Null,
    Error(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(n) => write!(f, "{}", n),
            Object::Char(c) => write!(f, "{}", c),
            Object::Str(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Function(func) => write!(f, "{}", func),
            Object::Builtin(bfunc) => write!(f, "{}", bfunc),
            Object::Array(arr) => {
                let elements = arr.iter().map(ToString::to_string).collect::<Vec<String>>();
                write!(f, "[{}]", elements.join(", "))
            }
            Object::Return(obj) => write!(f, "{}", obj),
            Object::Null => write!(f, "NULL"),
            Object::Error(err) => write!(f, "Err => {}", err),
        }
    }
}

impl Object {
    pub fn kind(&self) -> &str {
        match self {
            Object::Integer(_) => "Integer",
            Object::Float(_) => "Float",
            Object::Char(_) => "Character",
            Object::Str(_) => "String",
            Object::Boolean(_) => "Boolean",
            Object::Function(_) => "Function",
            Object::Builtin(_) => "Builtin Function",
            Object::Array(_) => "Array",
            Object::Return(_) => "RETURN",
            Object::Null => "NULL",
            Object::Error(_) => "ERROR",
        }
    }

    pub fn errored(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}
