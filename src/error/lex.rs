use crate::{
    lexer::token::{IllegalByteError, LexError, LexErrorKind},
    parser::ast::Spanned,
};

pub fn print_validation_errors(errors: Vec<IllegalByteError>) {
    for error in errors {
        print_validation_error(error);
    }
}

fn print_validation_error(
    Spanned {
        offset,
        value: error,
    }: IllegalByteError,
) {
    println!(
        "Validation error at offset {offset}: Illegal byte 0x{:02X}",
        error
    );
}

pub fn print_lex_errors(errors: Vec<LexError>, program: &str) {
    for error in errors {
        print_lex_error(error, program);
    }
}

fn print_lex_error(
    Spanned {
        offset,
        value: error,
    }: LexError,
    program: &str,
) {
    let (line, col) = super::get_line_and_column(program, offset);
    super::show_line_with_error(program, offset);
    match error {
        LexErrorKind::UnterminatedString => {
            println!("Lex error at line {line}, column {col}: Unterminated string literal");
        }
        LexErrorKind::IllegalCharacter(c) => {
            println!("Lex error at line {line}, column {col}: Illegal character '{c}'",);
        }
        LexErrorKind::UnterminatedComment => {
            println!("Lex error at line {line}, column {col}: Unterminated comment");
        }
    }
}
