use std::env;
use std::io;
use std::error::Error;
mod modules;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
   
    // Check how many args we have
    if args.len() < 2 || args.is_empty() { 
        modules::functions::help();
    }

    // Variables to receive input and transform according to the args
    let transformation: &str = &args[1].to_lowercase();
    let mut user_string = String::new();
    let mut string_mutation = String::new();

    println!("Text to transform:");
    
    read_input(&mut user_string);
    
    transform(&user_string);

    // Output transformation
    println!("--------------------------- \n\
              Original text: {} \n\
              Transformed text: {} \n\
              ---------------------------", 
              user_string, string_mutation
    );
}
