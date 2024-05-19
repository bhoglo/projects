use std::{io, io::Read, io::Cursor, error::Error, fmt::Formatter};
use slug::slugify;
use csv::{ReaderBuilder, WriterBuilder};

#[derive(Copy, Clone)]
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
fn help() -> String {
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
   std::process::exit(1);
}

pub fn parse_args(args: Vec<String>) -> Result<Command, Box<dyn Error>> {
    // Check how many args we have
    if args.len() < 2 || args.is_empty() {
        help();
    }

    let transformation = args[1].parse::<Command>()?;
    return Ok(transformation)
}

pub fn read_input(transformation: Command) -> Result<String, Box<dyn Error>> {
  let mut user_string = String::new();

  println!("Text to transform:");
  match transformation {
      Command::Csv => io::stdin().read_to_string(&mut user_string)?,
      _ => io::stdin().read_line(&mut user_string)?
  };

  let user_input: String = String::from(user_string);

  return Ok(user_input);
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

fn print_csv(user_string: &str) -> Result<String, Box<dyn Error>> {
    if !validate_input(user_string)
    {
        return Err("Input was an empty string.".into());
    }

    let mut csv_buffer = Cursor::new(user_string);
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&mut csv_buffer);

    let mut string_buffer = Vec::new();
    {
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_writer(&mut string_buffer);
        for result in reader.records() {
            match result {
                Ok(record) => writer.write_record(&record)?,
                Err(error) => {
                    eprintln!("Error reading CSV from <stdin>: {}", error);
                    std::process::exit(1);
                }
            }
        }

        writer.flush()?;
    }

    let output = format!("{}", String::from_utf8(string_buffer).unwrap());

    Ok(output)
}

pub fn output_transformation(user_string: String, string_mutation: String) {
    // Output transformation
     let output: String = format!("--------------------------- \n\
               Original text: \n {} \
               Transformed text: \n {} \
               ---------------------------",
               user_string, string_mutation
     );

     println!("{}", output);
}
