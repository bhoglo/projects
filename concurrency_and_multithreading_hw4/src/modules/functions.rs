use std::{io, io::Read, error::Error, fmt::Formatter, sync::mpsc, thread};
use slug::slugify;
use prettytable::{Slice, Table};

#[derive(Copy, Clone)]
enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Csv,
    Exit,
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
            "exit" => Ok(Command::Exit),
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
struct CommandParseError {
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
            Transformation options: \n\
            \t- lowercase \n\
            \t- uppercase \n\
            \t- no-spaces \n\
            \t- slugify \n\
            \t- csv \n\
            \t- exit \n\
            ------------------------------");
}

fn exit() -> ! {
   eprintln!("Exiting...");
   std::process::exit(0);
}

fn parse_args(args: Vec<String>) -> Result<Command, Box<dyn Error>> {
    let command = args[1].parse::<Command>()?;
    return Ok(command)
}

fn read_channel_input(tx: mpsc::Sender<(Command, String)>) -> Result<String, Box<dyn Error>> {
    // Interactive input reading
    // TODO: LOOP
    println!("No input supplied, entering interactive mode.");
    help();
    println!("Please enter a command:");

    let mut user_string: String = String::new();
    loop {
        // Temporary line: remove later
        let command: Command = io::stdin().read_line(&mut user_string)?;

        match command {
            Command::Csv => io::stdin().read_to_string(&mut user_string)?,
            Command::Lowercase => user_string = user_string.to_lowercase(),
            Command::Uppercase => user_string = user_string.to_uppercase(),
            Command::NoSpaces => user_string = user_string.replace(" ", ""),
            Command::Slugify => {
                let slugified = slugify(&user_string);
                user_string = slugified;
            },
            Command::Exit => break,
            _ => io::stdin().read_line(&mut user_string)?
        };

        let user_input: String = String::from(user_string);
        let _ = tx.send((command, user_input.clone()));
    }
    return Ok(user_input)
}

fn read_cli(command: Command, user_input: String, tx: mpsc::Sender<(Command, String)>) -> Result<String, Box<dyn Error>> {
    let mut user_string = user_input.clone();

    println!("Text to transform:");
    if !user_string.is_empty() {
        println!("{}", user_string);
        let _ = tx.send((command, user_input.clone()));

        return Ok(user_input);
    }

    match command {
        Command::Csv => io::stdin().read_to_string(&mut user_string)?,
        _ => io::stdin().read_line(&mut user_string)?
    };

    let user_input: String = String::from(user_string);
    let _ = tx.send((command, user_input.clone()));

    return Ok(user_input);
}


pub fn run(args: Vec<String>) -> Result<String, Box<dyn Error>> {
    let (tx, rx): (mpsc::Sender<(Command, String)>, mpsc::Receiver<(Command, String)>) = mpsc::channel();
 
    let output = thread::spawn(move || {
        for request in rx {
            match transform(request) {
                Ok(string_mutation) => output_transformation(string_mutation),
                Err(error) => {
                    eprintln!("Error writing output: {}", error)
                }
            };
        }
    });

    match args.len() {
        2 | 3  => { // NON-INTERACTIVE MODE
            let command: Command = parse_args(args.clone())?;
            let user_input: String = { 
                match args.len() {
                    3 => args[2..].join(" "),
                    _ => String::new()
                }
            };

            let input = thread::spawn(move || {
                if let Err(error) = read_cli(command, user_input, tx) {
                    eprintln!("Error reading input: {}", error);
                }
            });

            let _ = input.join();
            let _ = output.join();
        } 
        _ => {
            // TOO MANY OR NO ARGS - INTERACTIVE MODE
            let input = thread::spawn(move || {
                if let Err(error) = read_channel_input(tx) {
                    eprintln!("Error reading input: {}", error);
                }
            });

            let _ = input.join();
            let _ = output.join();
        } 
    }

    return Ok("Success".to_string())
}

fn transform(output: (Command, String)) -> Result<String, Box<dyn Error>> {
    let command = output.0;
    let user_string = output.1;

    let result = match command {
        Command::Lowercase => to_lowercase(user_string),
        Command::Uppercase => to_uppercase(user_string),
        Command::NoSpaces => remove_spaces(user_string),
        Command::Slugify => to_slugify(user_string),
        Command::Csv => print_csv(user_string),
        Command::Exit => exit(),
    };
    
    return Ok(result?)
}

fn validate_input(valid_string: &str) -> bool {
    !valid_string.trim().is_empty()
}

fn to_lowercase(user_string: String) -> Result<String, Box<dyn Error>> {
    if validate_input(&user_string)
    {
        Ok(user_string.to_lowercase())
    } else {
        Err("Input was an empty string.".into())
    }
}

fn to_uppercase(user_string: String) -> Result<String, Box<dyn Error>> {
    if validate_input(&user_string)
    {
        Ok(user_string.to_uppercase())
    } else {
        Err("Input was an empty string.".into())
    }
}

fn remove_spaces(user_string: String) -> Result<String, Box<dyn Error>> {
    if validate_input(&user_string)
    {
        Ok(user_string.replace(" ", ""))
    } else {
        Err("Input was an empty string.".into())
    }
}

fn to_slugify(user_string: String) -> Result<String, Box<dyn Error>> {
    if validate_input(&user_string)
    {
        Ok(slugify(&user_string))
    } else {
        Err("Input was an empty string.".into())
    }
}

fn print_csv(user_string: String) -> Result<String, Box<dyn Error>> {
    if !validate_input(&user_string)
    {
        return Err("Input was an empty string.".into());
    }

    let table = Table::from_csv_string(&user_string)?;

    Ok(table.slice(..).to_string())
}

fn output_transformation(string_mutation: String) {
    // Output transformation
     let output: String = format!("\n\
               Transformed text: \n {} \n ",
               string_mutation
     );

     println!("{}", output);
}
