use std::{fmt::Display, u32};

use crate::{lace_lib, Object};

#[derive(Clone)]
pub enum BuiltinFunction {
    Echo,
    Read,
    Len,
    First,
    Split,
}

macro_rules! check_n_args {
    ($expected:expr,$received:expr) => {
        if $expected != $received {
            return Object::Error(format!(
                "Incorrect number of arguments provided. Expected {}, Received {}.",
                $expected, $received
            ));
        }
    };
}

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut disp = String::from("Builtin ");
        let func = match self {
            BuiltinFunction::Echo => "echo",
            BuiltinFunction::Read => "read",
            BuiltinFunction::Len => "len",
            BuiltinFunction::First => "first",
            BuiltinFunction::Split => "split",
        };
        disp.push_str(func);
        write!(f, "{}", disp)
    }
}

impl BuiltinFunction {
    pub fn apply(&self, args: Vec<Object>) -> Object {
        check_n_args!(self.expected_args(), args.len() as u32);

        match self {
            BuiltinFunction::Echo => lace_lib::std::echo(args[0].clone()),
            BuiltinFunction::Read => lace_lib::std::read(args[0].clone()),
            BuiltinFunction::Len => lace_lib::std::len(args[0].clone()),
            BuiltinFunction::First => lace_lib::std::first(args[0].clone()),
            BuiltinFunction::Split => lace_lib::str::split(args[0].clone(), args[1].clone()),
        }
    }

    fn expected_args(&self) -> u32 {
        match self {
            BuiltinFunction::Echo
            | BuiltinFunction::Read
            | BuiltinFunction::Len
            | BuiltinFunction::First => 1,
            BuiltinFunction::Split => 2,
        }
    }

    pub fn try_builtin(name: &str) -> Option<Object> {
        let func = match name {
            "echo" => BuiltinFunction::Echo,
            "read" => BuiltinFunction::Read,
            "len" => BuiltinFunction::Len,
            "first" => BuiltinFunction::First,
            "split" => BuiltinFunction::Split,
            "write" => todo!(),
            _ => {
                return None;
            }
        };

        Some(Object::Builtin(func))
    }
}
