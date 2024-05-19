use std::env;
mod modules;
use crate::modules::functions::*;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    // Variables to receive input and transform according to the args
    let transformation: Command = parse_args(args).expect("Failed to parse arguements.");
    let user_string: String = read_input(transformation.clone()).expect("Failed to read command line input.");
    let string_mutation: String = transform(transformation, &user_string).expect("Failed to transform user string.");

    // Output transformation
    output_transformation(user_string, string_mutation);
}
