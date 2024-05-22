pub mod prompt;

use std::{
    io::{self, Stderr, Stdin, Stdout, Write},
    process,
};

use lace_eval::{object::Object, Eval};
use lace_lexer::Lexer;
use lace_parser::Parser;

use self::prompt::ReplPrompt;

pub fn run_repl() {
    let (stdin, mut stdout, stderr): (Stdin, Stdout, Stderr) =
        (io::stdin(), io::stdout(), io::stderr());

    let mut prompt = ReplPrompt::default();

    write!(&stdout, "{}", prompt.logo).unwrap();

    let mut eval = Eval::default();

    loop {
        write!(&stdout, "{}{}\x1b[0m", prompt.colour(), prompt.symbol).unwrap();

        stdout.flush().unwrap();

        let mut input = String::new();

        if let Err(err) = stdin.read_line(&mut input) {
            writeln!(&stderr, "Error: {}", err).unwrap();
            process::exit(1);
        };

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        match parser.errors.is_empty() {
            true => {
                prompt.works();
                let v = eval.eval(program);
                if let Object::Error(err) = v {
                    writeln!(&stderr, "{}", err).unwrap();
                    prompt.errored();
                } else if let Object::Null = v {
                    // let the prompt remain empty if there is expression is evaluated to null
                } else {
                    writeln!(&stdout, "{}", v).unwrap();
                }
            }
            false => {
                prompt.errored();
                parser.errors.iter().for_each(|e| {
                    println!("{}", e.log_err());
                })
            }
        }
    }
}
