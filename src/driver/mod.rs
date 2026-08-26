use crate::{error, lexer};

mod print;

pub fn run() {
    let Config { program, mode } = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let tokens = match lexer::lex(&program) {
        Ok(tokens) => tokens,
        Err(lex_errors) => {
            eprintln!("Compilation failed: lexical analysis failed");
            error::lex::print_lex_error(lex_errors, &program);
            std::process::exit(1);
        }
    };
}

fn parse_args<'a>() -> Result<Config, CliError> {
    let args = std::env::args();
    let mut filename = None;
    let mut mode = None;
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

    let program = match filename {
        Some(f) => std::fs::read_to_string(&f).map_err(|_| CliError::UnknownFile(f))?,
        None => return Err(CliError::MissingFilename),
    };

    Ok(Config { program, mode })
}

pub struct Config {
    program: String,
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
    UnknownFile(String),
    UnknownOption(String),
    MultipleModesSpecified,
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingFilename => write!(f, "Missing filename"),
            CliError::UnknownFile(filename) => write!(f, "File not found: {}", filename),
            CliError::UnknownOption(option) => write!(f, "Unknown option: {}", option),
            CliError::MultipleModesSpecified => write!(f, "Multiple modes specified"),
        }
    }
}
