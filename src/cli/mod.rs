use std::path::PathBuf;

use crate::{interpretter, repl};

#[derive(Default)]
pub struct InterpreterArgs {
    file: Option<PathBuf>,
}

impl From<Vec<String>> for InterpreterArgs {
    fn from(value: Vec<String>) -> Self {
        let len = value.len();
        if len <= 1 {
            Self::default()
        } else if len > 2 {
            panic!("Too many arguments received")
        } else {
            Self {
                file: Some(PathBuf::from(&value[1])),
            }
        }
    }
}

impl InterpreterArgs {
    pub fn run(&self) {
        match &self.file {
            Some(file_path) => {
                if let Err(err) = interpretter::run_interpreter(file_path.clone()) {
                    eprintln!("{err}");
                }
            }
            None => repl::run_repl(),
        }
    }
}
