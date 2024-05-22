use std::env;

use lace::cli::InterpreterArgs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let interpreter = InterpreterArgs::from(args);
    interpreter.run();
}
