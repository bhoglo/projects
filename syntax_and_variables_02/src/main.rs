use std::env;
use std::io;
use slug::slugify;


/*
 * Displays acceptable arguements
 */
fn help() {
    println!("------------------------------ \n\
             Usage: ./syntax_and_variables_02 <transformation> \n\
             ------------------------------ \n\
             Transformation options: \n\
             \t- lowercase \n\
             \t- uppercase \n\
             \t- no-spaces \n\
             \t- slugify \n\
             ------------------------------");
}

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
   
    // Check how many args we have
    if args.len() < 2 || args.is_empty() { 
        help();
        return; // exit code
    }

    // Variables to receive input and transform according to the args
    let transformation: &str = &args[1].to_lowercase();
    let mut user_string = String::new();
    let mut string_mutation = String::new();

    println!("Text to transform:");
    io::stdin().read_line(&mut user_string).expect("Failed to read line.");

    match transformation {
        "lowercase" => string_mutation.push_str(&user_string.to_lowercase()),
        "uppercase" => string_mutation.push_str(&user_string.to_uppercase()),
        "no-spaces" => string_mutation.push_str(&user_string.replace(" ", "")),
        "slugify" => string_mutation.push_str(&slugify(user_string.clone())),
        _=> help(), // Display help if transformation doesn't exist
    };

    // Output transformation
    println!("--------------------------- \n\
              Original text: {} \n\
              Transformed text: {} \n\
              ---------------------------", 
              user_string, string_mutation
    );
}
