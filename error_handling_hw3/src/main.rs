use std::env;
mod modules;
use crate::modules::functions::*;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    // Variables to receive input and transform according to the args
    let transformation: Command = parse_args(args);
    let mut user_string = read_input();
    let mut string_mutation = transform(transformation, &user_string);

    // Output transformation
    output_transformation(user_string, string_mutation);
}
