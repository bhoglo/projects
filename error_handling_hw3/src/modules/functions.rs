use slug::slugify;
use std::error::Error;

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

// Box of information containing variables
pub struct CommandParseError {
    // variable of type string
    invalid_command: String,
}

impl Error for CommandParseError {}

/*
 * https://doc.rust-lang.org/std/fmt/trait.Display.html
 */
impl std::fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid command line arguement: {}", self.invalid_command)
    }
}

/*
* SUPPORT FUNCTIONS
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
            ------------------------------");
}

fn read_input(mut user_string) -> Result<String, Box<dyn Error>> {
  let input_from_user = io::stdin().read_line(&mut user_string)?
  return Ok(input_from_user);
}

fn tranform(String) -> Result<String, Box<dyn Error>> {
     match transformation {
         "lowercase" => toLowercase(&user_string),
         "uppercase" => toUppercase(&user_string),
         "no-spaces" => removeSpaces(&user_string),
         "slugify" => toSlugify(&user_string),
          _=> help(), // Display help if transformation doesn't exist
      };
}

fn validInput(valid_string: &str) -> bool {
    !valid_string.trim().is_empty()
}

fn toLowercase(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validInput(user_string)
    {
        Ok(user_string.to_lowercase())
    } else {
        Err("Input was an empty string.")
    }
}

fn toUppercase(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validInput(user_string)
    {
        Ok(user_string.to_uppercase())
    } else {
        Err("Input was an empty string.")
    }
}

fn removeSpaces(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validInput(user_string)
    {
        Ok(user_string.replace(" ", ""))
    } else {
        Err("Input was an empty string.")
    }
}

fn toSlugify(user_string: &str) -> Result<String, Box<dyn Error>> {
    if validInput(user_string)
    {
        Ok(slugify(user_string.clone()))
    } else {
        Err("Input was an empty string.")
    }
}
