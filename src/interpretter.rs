use std::{fs, path::PathBuf};

use lace_eval::Eval;
use lace_lexer::Lexer;
use lace_parser::Parser;

macro_rules! check_ext {
    ($source:expr) => {
        let ext = &$source.extension();
        match ext {
            Some(a) => {
                if a.to_str() != Some("lace") {
                    return Err("Expected a .lace file".to_string());
                }
            }
            None => return Err("Expected a .lace file".to_string()),
        }
    };
}
pub fn run_interpreter(source: PathBuf) -> Result<(), String> {
    if !source.is_file() {
        return Err(format!("Couldn't find {:?}", source));
    }

    check_ext!(&source);

    let code = match fs::read_to_string(&source) {
        Ok(code) => code,
        Err(_) => {
            return Err(format!("Failed to read from {:?}", source));
        }
    };

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if parser.errors.is_empty() {
        let mut evaluator = Eval::new();
        evaluator.eval(program);
    } else {
        parser.log_errors();
    }

    Ok(())
}
