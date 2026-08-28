use crate::{error, lexer};

mod print;

pub fn run() {
    let Config { filename, mode } = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let program = match std::fs::read(&filename) {
        Ok(program) => program,
        Err(e) => {
            println!(
                "Compilation failed: could not read file '{}': {}",
                filename, e
            );
            std::process::exit(1);
        }
    };

    let tokens = match lexer::lex(&program) {
        Ok(tokens) => tokens,
        Err(lex_errors) => {
            error::lex::print_lex_error(lex_errors, &program);
            println!("Compilation failed: lexical analysis failed");
            std::process::exit(1);
        }
    };

    if let Some(mode) = mode
        && mode == Mode::Lex
    {
        print::lex::print_tokens(tokens);
        println!("Compilation succeeded: lexical analysis complete");
        return;
    }
}

fn parse_args<'a>() -> Result<Config, CliError> {
    let mut args = std::env::args();
    let mut filename = None;
    let mut mode = None;
    args.next(); // skip the program name
    for argument in args {
        if argument.starts_with("-") {
            if mode.is_some() {
                return Err(CliError::MultipleModesSpecified);
            }
            match argument.as_str() {
                "-l" => mode = Some(Mode::Lex),
                "-p" => mode = Some(Mode::Parse),
                "-t" => mode = Some(Mode::Typecheck),
                "-i" => mode = Some(Mode::IR),
                "-s" => mode = Some(Mode::Assembly),
                _ => return Err(CliError::UnknownOption(argument)),
            }
        } else {
            filename = Some(argument);
        }
    }

    let filename = match filename {
        Some(filename) => filename,
        None => return Err(CliError::MissingFilename),
    };

    Ok(Config { filename, mode })
}

pub struct Config {
    filename: String,
    mode: Option<Mode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Lex,
    Parse,
    Typecheck,
    IR,
    Assembly,
}

enum CliError {
    MissingFilename,
    UnknownOption(String),
    MultipleModesSpecified,
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingFilename => write!(f, "Missing filename"),
            CliError::UnknownOption(option) => write!(f, "Unknown option: {}", option),
            CliError::MultipleModesSpecified => write!(f, "Multiple modes specified"),
        }
    }
}
