use crate::{
    Spanned,
    lexer::token::{LexError, LexErrorKind},
};

pub fn print_lex_errors(errors: Vec<LexError>, program: &[u8]) {
    for error in errors {
        print_lex_error(error, program);
    }
}

fn print_lex_error(
    Spanned {
        offset,
        value: error,
    }: LexError,
    program: &[u8],
) {
    let (line, col) = super::get_line_and_column(program, offset);
    super::show_line_with_error(program, offset);
    match error {
        LexErrorKind::UnterminatedString => {
            println!("Lex error at line {line}, column {col}: Unterminated string literal");
        }
        LexErrorKind::IllegalCharacter(c) => {
            println!("Lex error at line {line}, column {col}: Illegal character '{c}'");
        }
        LexErrorKind::UnterminatedComment => {
            println!("Lex error at line {line}, column {col}: Unterminated comment");
        }
        LexErrorKind::IllegalByte(b) => {
            println!("Lex error at line {line}, column {col}: Illegal byte 0x{b:02X}");
        }
    }
    println!();
}
