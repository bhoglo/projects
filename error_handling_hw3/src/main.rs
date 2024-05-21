use std::env;
mod modules;
use crate::modules::functions::run;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    run(args).unwrap_or_else(|error| {
        eprintln!("Error running program: {}", error);
        std::process::exit(1);
    });
}
