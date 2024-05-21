use std::env;
mod modules;
use crate::modules::functions::run;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(transmutation) => println!("Program status: {}", transmutation),
        Err(error) => {
            eprintln!("Error running program: {}", error);
            std::process::exit(1);
        }
    }
}
