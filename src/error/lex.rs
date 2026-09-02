use std::fmt::Display;

use crate::lexer::token::{IllegalByteError, LexError};

pub fn print_validation_error(errors: Vec<IllegalByteError>) {
    for error in errors {
        println!("{error}");
    }
    println!("Compilation failed: lexical analysis failed");
}

pub fn print_lex_error(errors: Vec<LexError>, program: &str) {
    for error in errors {
        println!("Lexical error: {error}");
    }
}

impl Display for IllegalByteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Illegal byte 0x{:02X} at offset {}",
            self.byte, self.offset
        )
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString(offset) => {
                write!(f, "Unterminated string literal at offset {offset}")
            }
            Self::IllegalCharacter(offset, c) => {
                write!(f, "Illegal character '{c}' at offset {offset}")
            }
            Self::UnterminatedComment(offset) => {
                write!(f, "Unterminated comment at offset {offset}")
            }
        }
    }
}
