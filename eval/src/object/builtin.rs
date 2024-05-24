use std::fmt::Display;

use crate::{lace_lib, Object};

#[derive(PartialEq, Clone)]
pub enum BuiltinFunction {
    Kind,
    Write,
    Read,
    Len,
    First,
    Split,
    Append,
    StripStart,
    StripEnd,
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
            BuiltinFunction::Kind => "kind",
            BuiltinFunction::Write => "write",
            BuiltinFunction::Read => "read",
            BuiltinFunction::Len => "len",
            BuiltinFunction::First => "first",
            BuiltinFunction::Split => "split",
            BuiltinFunction::Append => "append",
            BuiltinFunction::StripStart => "strip_start",
            BuiltinFunction::StripEnd => "strip_end",
        };
        disp.push_str(func);
        write!(f, "{}", disp)
    }
}

impl BuiltinFunction {
    pub fn apply(&self, args: Vec<Object>) -> Object {
        check_n_args!(self.expected_args(), args.len() as u32);

        match self {
            BuiltinFunction::Kind => lace_lib::std::kind(args[0].clone()),
            BuiltinFunction::Write => lace_lib::std::write(args[0].clone()),
            BuiltinFunction::Read => lace_lib::std::read(args[0].clone()),
            BuiltinFunction::Len => lace_lib::std::len(args[0].clone()),
            BuiltinFunction::First => lace_lib::std::first(args[0].clone()),
            BuiltinFunction::Split => lace_lib::str::split(args[0].clone(), args[1].clone()),
            BuiltinFunction::Append => lace_lib::array::append(args[0].clone(), args[1].clone()),
            BuiltinFunction::StripStart => {
                lace_lib::std::strip_start(args[0].clone(), args[1].clone())
            }
            BuiltinFunction::StripEnd => lace_lib::std::strip_end(args[0].clone(), args[1].clone()),
        }
    }

    fn expected_args(&self) -> u32 {
        match self {
            BuiltinFunction::Kind
            | BuiltinFunction::Write
            | BuiltinFunction::Read
            | BuiltinFunction::Len
            | BuiltinFunction::First => 1,
            BuiltinFunction::Split
            | BuiltinFunction::Append
            | BuiltinFunction::StripStart
            | BuiltinFunction::StripEnd => 2,
        }
    }

    pub fn try_builtin(name: &str) -> Option<Object> {
        let func = match name {
            "kind" => BuiltinFunction::Kind,
            "writeln" => BuiltinFunction::Write,
            "read" => BuiltinFunction::Read,
            "len" => BuiltinFunction::Len,
            "first" => BuiltinFunction::First,
            "split" => BuiltinFunction::Split,
            "append" => BuiltinFunction::Append,
            "strip_start" => BuiltinFunction::StripStart,
            "strip_end" => BuiltinFunction::StripEnd,
            _ => {
                return None;
            }
        };

        Some(Object::Builtin(func))
    }
}
