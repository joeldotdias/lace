use std::fmt::Display;

use crate::{lace_lib, Object};

#[derive(Clone)]
pub enum BuiltinFunction {
    Len,
}

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut disp = String::from("Builtin ");
        let func = match self {
            BuiltinFunction::Len => "len",
        };
        disp.push_str(func);
        write!(f, "{}", disp)
    }
}

impl BuiltinFunction {
    pub fn apply(&self, args: Vec<Object>) -> Object {
        let len = args.len();
        match self {
            BuiltinFunction::Len => {
                if len != 1 {
                    return Object::Error(format!(
                        "Incorrect number of arguments provided. Got {}, Expected 1",
                        len
                    ));
                }
                lace_lib::std::len(args[0].clone())
            }
        }
    }

    pub fn try_builtin(name: &str) -> Option<Object> {
        let func = match name {
            "len" => BuiltinFunction::Len,
            "write" => todo!(),
            _ => {
                return None;
            }
        };

        Some(Object::Builtin(func))
    }
}
