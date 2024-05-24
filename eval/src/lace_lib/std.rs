use std::io::{self, Write};

use crate::object::Object;

pub fn kind(obj: Object) -> Object {
    Object::Str(obj.kind().into())
}

pub fn write(obj: Object) -> Object {
    println!("{}", obj);
    Object::Null
}

pub fn read(obj: Object) -> Object {
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    if let Err(_) = io::stdin().read_line(&mut buffer) {
        return Object::Error("Failed to read from stdin".into());
    };

    match obj {
        Object::Integer(_) => Object::Integer(buffer.parse::<i64>().expect("Expected an integer")),
        Object::Float(_) => Object::Float(
            buffer
                .parse::<f64>()
                .expect("Expected a floating point number"),
        ),
        Object::Char(_) => {
            if let 1 = buffer.len() {
                Object::Char(buffer.chars().nth(0).unwrap())
            } else {
                Object::Error("More than one characters received".into())
            }
        }
        Object::Str(_) => Object::Str(buffer),
        _ => Object::Error(format!("No associated function read() for {}", obj.kind())),
    }
}

pub fn len(obj: Object) -> Object {
    let l = match obj {
        Object::Str(s) => s.len(),
        Object::Array(arr) => arr.len(),
        _ => {
            return Object::Error(format!(
                "{} does not have any associated function len()",
                obj.kind()
            ));
        }
    };

    Object::Integer(l as i64)
}

pub fn strip_start(obj: Object, strip: Object) -> Object {
    match obj.clone() {
        Object::Str(s) => match strip {
            Object::Str(stripable) => {
                let mdf = match s.strip_prefix(&stripable) {
                    Some(nw) => nw,
                    None => &s,
                };
                Object::Str(mdf.into())
            }
            Object::Integer(i) => {
                let mdf = &s[(i as usize)..];
                Object::Str(mdf.into())
            }
            _ => Object::Error(format!("{} cannot be stripped from a String", obj.kind())),
        },
        Object::Array(arr) => match strip {
            Object::Array(stripable) => {
                let strip_len = stripable.len();
                let mdf = if &arr[..strip_len] == &stripable {
                    &arr[strip_len..]
                } else {
                    &arr
                };
                Object::Array(mdf.into())
            }
            Object::Integer(i) => {
                let mdf = &arr[(i as usize)..];
                Object::Array(mdf.into())
            }
            _ => Object::Error(format!("{} cannot be stripped from an Array", obj.kind())),
        },

        _ => Object::Error(format!(
            "No associated function strip_start() for {}",
            obj.kind()
        )),
    }
}

pub fn strip_end(obj: Object, strip: Object) -> Object {
    match obj.clone() {
        Object::Str(s) => match strip {
            Object::Str(stripable) => {
                let mdf = match s.strip_suffix(&stripable) {
                    Some(nw) => nw,
                    None => &s,
                };
                Object::Str(mdf.into())
            }
            Object::Integer(i) => {
                let mdf = &s[..(s.len() - i as usize)];
                Object::Str(mdf.into())
            }
            _ => Object::Error(format!("{} cannot be stripped from a ", obj.kind())),
        },
        Object::Array(arr) => match strip {
            Object::Array(stripable) => {
                let strip_len = stripable.len();
                let arr_len = arr.len();
                let mdf = if &arr[(arr_len - strip_len)..] == &stripable {
                    &arr[..(arr.len() - strip_len)]
                } else {
                    &arr
                };
                Object::Array(mdf.into())
            }
            Object::Integer(i) => {
                let mdf = &arr[..(i as usize)];
                Object::Array(mdf.into())
            }
            _ => Object::Error(format!("{} cannot be stripped from an Array", obj.kind())),
        },
        _ => Object::Error(format!(
            "No associated function strip_start() for {}",
            obj.kind()
        )),
    }
}

pub fn first(obj: Object) -> Object {
    match obj {
        Object::Str(s) => match s.chars().nth(0) {
            Some(ch) => Object::Char(ch),
            None => Object::Null,
        },
        Object::Array(a) => {
            if a.len() == 0 {
                return Object::Error("Array is empty".into());
            }
            a[0].clone()
        }
        _ => Object::Error(format!(
            "{} does not have any associated function first()",
            obj.kind()
        )),
    }
}
