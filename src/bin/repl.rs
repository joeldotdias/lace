use std::{
    io::{self, Stderr, Stdin, Stdout, Write},
    process,
};

use lace_eval::{object::Object, Eval};
use lace_lexer::Lexer;
use lace_parser::Parser;

#[derive(Default)]
enum PromptColour {
    #[default]
    Works,
    Error,
}

impl PromptColour {
    fn colour(&self) -> &str {
        match self {
            PromptColour::Works => "\x1b[92m",
            PromptColour::Error => "\x1b[91m",
        }
    }
}

const LOGO: &str = "
___        ___
.'|        .'|=|`.     .'|=|_.'   .'|=|_.'
.'  |      .'  | |  `. .'  |      .'  |  ___
|   |      |   |=|   | |   |      |   |=|_.'
|   |  ___ |   | |   | `.  |  ___ |   |  ___
|___|=|_.' |___| |___|   `.|=|_.' |___|=|_.'


";

const PROMPT: &str = ">> ";

fn main() {
    let (stdin, mut stdout, stderr): (Stdin, Stdout, Stderr) =
        (io::stdin(), io::stdout(), io::stderr());
    let mut prompt_colour = PromptColour::default();

    write!(&stdout, "{}", LOGO).unwrap();

    loop {
        write!(&stdout, "{}{}\x1b[0m", prompt_colour.colour(), PROMPT).unwrap();

        stdout.flush().unwrap();

        let mut input = String::new();

        if let Err(err) = stdin.read_line(&mut input) {
            writeln!(&stderr, "Error: {}", err).unwrap();
            process::exit(1);
        };

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut eval = Eval {};

        let program = parser.parse_program();

        match parser.errors.is_empty() {
            true => {
                prompt_colour = PromptColour::Works;
                let v = eval.eval(program);
                if let Object::Error(err) = v {
                    writeln!(&stderr, "{}", err).unwrap();
                    prompt_colour = PromptColour::Error;
                } else {
                    writeln!(&stdout, "{}", v).unwrap();
                }
            }
            false => {
                prompt_colour = PromptColour::Error;
                parser.errors.iter().for_each(|e| {
                    println!("{}", e.log_err());
                })
            }
        }
    }
}
