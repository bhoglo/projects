use std::env;
mod modules;
use crate::modules::functions::run;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(status) => println!("Program status: {}", status),
        Err(error) => {
            eprintln!("Error running program: {}", error);
            std::process::exit(1);
        }
    }
}
