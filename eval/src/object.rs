use std::fmt::Display;

#[derive(Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Char(char),
    Str(String),
    Boolean(bool),
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
            Object::Return(obj) => write!(f, "{}", obj),
            Object::Null => write!(f, "NULL"),
            Object::Error(err) => write!(f, "Err => {}", err),
        }
    }
}

impl Object {
    pub fn kind(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Float(_) => "FLOAT",
            Object::Char(_) => "CHAR",
            Object::Str(_) => "STR",
            Object::Boolean(_) => "BOOLEAN",
            Object::Return(_) => "RETURN",
            Object::Null => "NULL",
            Object::Error(_) => "ERROR",
        }
    }
}
