use std::process::ExitCode;

use bumpalo::Bump;

use crate::{error, lexer, parser, typechecker};

mod print;

#[must_use]
pub fn run() -> ExitCode {
    let Config { filename, mode } = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {e:?}");
            return ExitCode::from(1);
        }
    };

    let program = match std::fs::read(&filename) {
        Ok(program) => program,
        Err(e) => {
            println!("Compilation failed: could not read file '{filename}': {e}");
            return ExitCode::from(1);
        }
    };

    let tokens = match lexer::lex(&program) {
        Ok(tokens) => tokens,
        Err(lex_errors) => {
            if error::lex::print_lex_errors(lex_errors, &program).is_err() {
                println!("Compilation failed: could not print lex errors");
            }
            println!("Compilation failed: lexical analysis failed");
            return ExitCode::from(1);
        }
    };

    if mode == Some(Mode::Lex) {
        if print::lex::print_tokens(tokens).is_err() {
            println!("Compilation failed: could not print tokens");
            return ExitCode::from(1);
        }
        println!("Compilation succeeded: lexical analysis complete");
        return ExitCode::from(0);
    }

    let ast_alloc = Bump::new(); // this is mutable, thanks RefCell
    let parsed_program = match parser::parse(&ast_alloc, tokens) {
        Ok(parsed_program) => parsed_program,
        Err(parse_errors) => {
            if error::parse::print_parse_error(parse_errors, &program).is_err() {
                println!("Compilation failed: could not print parse errors");
            }
            println!("Compilation failed: parsing failed");
            return ExitCode::from(1);
        }
    };

    if mode == Some(Mode::Parse) {
        if print::parse::print_sexp(parsed_program).is_err() {
            println!("Compilation failed: could not print parsed AST");
            return ExitCode::from(1);
        }
        println!("Compilation succeeded: parsing complete");
        return ExitCode::from(0);
    }

    let typeck_alloc = Bump::with_capacity(ast_alloc.allocated_bytes());
    let typed_program = match typechecker::typecheck(&typeck_alloc, parsed_program) {
        Ok(typed_program) => typed_program,
        Err(type_errors) => {
            if error::typecheck::print_type_error(type_errors, &program).is_err() {
                println!("Compilation failed: could not print type errors");
            }
            println!("Compilation failed: typechecking failed");
            return ExitCode::from(1);
        }
    };

    if mode == Some(Mode::Typecheck) {
        if print::typecheck::print_typed_program(typed_program).is_err() {
            println!("Compilation failed: could not print typed AST");
            return ExitCode::from(1);
        }
        println!("Compilation succeeded: typechecking complete");
        return ExitCode::from(0);
    }

    todo!()
}

fn parse_args() -> Result<Config, CliError> {
    let mut args = std::env::args();
    let mut filename = None;
    let mut mode = None;
    args.next(); // skip the program name
    for argument in args {
        if argument.starts_with('-') {
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
            if filename.is_some() {
                return Err(CliError::MultipleFilesSpecified);
            }
            filename = Some(argument);
        }
    }

    let Some(filename) = filename else {
        return Err(CliError::MissingFilename);
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
    MultipleFilesSpecified,
    MultipleModesSpecified,
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFilename => write!(f, "Missing filename"),
            Self::UnknownOption(option) => write!(f, "Unknown option: {option}"),
            Self::MultipleFilesSpecified => write!(f, "Multiple files specified"),
            Self::MultipleModesSpecified => write!(f, "Multiple modes specified"),
        }
    }
}
