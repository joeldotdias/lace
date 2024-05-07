use std::{io::{self, Stderr, Stdin, Stdout, Write}, process};

use lace::lexer::Lexer;

#[derive(Default)]
enum PromptColour {
    #[default]
    Works,
    Error,
}

impl PromptColour {
    fn colour(&self) -> &str {
        match self {
            PromptColour::Works =>  "\x1b[92m",
            PromptColour::Error =>  "\x1b[91m",
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
    let (stdin, mut stdout, stderr): (Stdin, Stdout, Stderr) = (io::stdin(), io::stdout(), io::stderr());
    let mut prompt_colour = PromptColour::default();

    write!(&stdout, "{}",LOGO).unwrap();

    loop {
        // write!(&stdout, "\x1b[93m{}\x1b[0m", PROMPT).unwrap();
        write!(&stdout, "{}{}\x1b[0m", prompt_colour.colour(), PROMPT).unwrap();

        stdout.flush().unwrap();

        let mut input = String::new();

        if let Err(err) = stdin.read_line(&mut input) {
                    writeln!(&stderr, "Error: {}", err).unwrap();
                    process::exit(1);
        };

        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_token();

            if token.reached_eof() {
                break;
            }

            match writeln!(&stdout, "{}", token) {
                Ok(_) => { prompt_colour = PromptColour::Works; },
                Err(_) => { prompt_colour = PromptColour::Error; },
            };
        }
    }
}
