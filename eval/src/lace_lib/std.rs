use std::io::{self, Write};

use crate::object::Object;

pub fn echo(obj: Object) -> Object {
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
