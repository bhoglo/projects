use std::io;
use slug::slugify;
use std::error::Error;
use std::fmt::Formatter;

pub enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Csv,
}

/*
 * https://doc.rust-lang.org/std/str/trait.FromStr.html
 */
impl std::str::FromStr for Command {
    type Err = CommandParseError;

    fn from_str(command_input: &str) -> Result<Self, Self::Err> {
        match command_input {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "slugify" => Ok(Command::Slugify),
            "csv" => Ok(Command::Csv),
            _ => Err(CommandParseError {
                // Building out the new error of type CommandParseError
                invalid_command: command_input.to_string(),
            }),
        }
    }
}

/* 
 * Box of information containing variables,
 * and Debug is necessary implementation for type Error
 */
#[derive(Debug)]
pub struct CommandParseError {
    // variable of type string
    invalid_command: String,
}

impl Error for CommandParseError {}

/*
 * https://doc.rust-lang.org/std/fmt/trait.Display.html
 * Display needed for type Errors
 */
impl std::fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid command line arguement: {}", self.invalid_command)
    }
}

/*
*                    SUPPORT FUNCTIONS
*/
fn help() {
   eprintln!("------------------------------ \n\
            Usage: ./error_handling_hw3 <transformation> \n\
            ------------------------------ \n\
            Transformation options: \n\
            \t- lowercase \n\
            \t- uppercase \n\
            \t- no-spaces \n\
            \t- slugify \n\
            \t- csv \n\
            ------------------------------");
    std::process::exit(0); // exit code
}

pub fn parse_args(args: Vec<String>) -> Result<Command, Box<dyn Error>> {
    // Check how many args we have
    if args.len() < 2 || args.is_empty() {
        help();
    }

    let transformation = args[1].parse::<Command>()?;
    return Ok(transformation)
}

pub fn read_input() -> Result<String, Box<dyn Error>> {
  let mut user_string = String::new();

  println!("Text to transform:");
  let input_from_user = io::stdin().read_line(&mut user_string)?;
  
  return Ok(input_from_user.to_string());
}

pub fn transform(transformation: Command, user_string: &str) -> Result<String, Box<dyn Error>> {
    let result = match transformation {
        Command::Lowercase => to_lowercase(user_string),
        Command::Uppercase => to_uppercase(user_string),
        Command::NoSpaces => remove_spaces(user_string),
        Command::Slugify => to_slugify(user_string),
        Command::Csv => print_csv(user_string),
    };
    
    return Ok(result?)
}

fn validate_input(valid_string: &str) -> bool {
    !valid_string.trim().is_empty()
}

fn to_lowercase(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validate_input(user_string)
    {
        Ok(user_string.to_lowercase())
    } else {
        Err("Input was an empty string.".into())
    }
}

fn to_uppercase(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validate_input(user_string)
    {
        Ok(user_string.to_uppercase())
    } else {
        Err("Input was an empty string.".into())
    }
}

fn remove_spaces(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validate_input(user_string)
    {
        Ok(user_string.replace(" ", ""))
    } else {
        Err("Input was an empty string.".into())
    }
}

fn to_slugify(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validate_input(user_string)
    {
        Ok(slugify(user_string))
    } else {
        Err("Input was an empty string.".into())
    }
}

// TODO: Remove underscore infront of user_string when implementing.
fn print_csv(_user_string: &str) -> Result<String, Box<dyn Error>> {
    todo!();
}

pub fn output_transformation(user_string: &str, string_mutation: &str) {
    // Output transformation
     println!("--------------------------- \n\
               Original text: {} \n\
               Transformed text: {} \n\
               ---------------------------",
               user_string, string_mutation
     );
}
