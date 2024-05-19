use std::env;
mod modules;
use crate::modules::functions::*;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    // Variables to receive input and transform according to the args
    let transformation: Command = parse_args(args).unwrap_or_else(|error| {
        eprintln!("Unable to parse arguments: {}", error);
        help();
        std::process::exit(1);
    });

    let user_string: String = read_input(transformation.clone()).unwrap_or_else(|error| {
        eprintln!("Failed to read stdin: {}", error);
        std::process::exit(1);
    });

    let string_mutation: String = transform(transformation, &user_string).unwrap_or_else(|error| {
        eprintln!("Failed to transform input: {}", error);
        std::process::exit(1);
    });

    // Output transformation
    output_transformation(user_string, string_mutation);
}
