use std::fmt::Display;

use crate::lexer::token::LexError;

pub fn print_lex_error(errors: Vec<LexError>, program: &str) {
    for error in errors {
        println!("Lexical error: {}", error);
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnterminatedString(offset) => {
                write!(f, "Unterminated string literal at offset {}", offset)
            }
            LexError::IllegalCharacter(offset, c) => {
                write!(f, "Illegal character '{}' at offset {}", c, offset)
            }
        }
    }
}
