use std::io::{self, BufWriter, Write};

use crate::{
    Spanned,
    lexer::token::{LexError, LexErrorKind},
};

pub fn print_lex_errors(errors: Vec<LexError>, program: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    for error in errors {
        print_lex_error(&mut writer, error, program)?;
    }
    writer.flush()?;
    Ok(())
}

fn print_lex_error(
    writer: &mut impl Write,
    Spanned {
        offset,
        value: error,
    }: LexError,
    program: &[u8],
) -> io::Result<()> {
    let (line, col) = super::get_line_and_column(program, offset);
    super::show_line_with_error(writer, program, offset)?;
    match error {
        LexErrorKind::UnterminatedString => {
            writeln!(
                writer,
                "Lex error at line {line}, column {col}: Unterminated string literal"
            )?;
        }
        LexErrorKind::IllegalCharacter(c) => {
            writeln!(
                writer,
                "Lex error at line {line}, column {col}: Illegal character '{c}'"
            )?;
        }
        LexErrorKind::UnterminatedComment => {
            writeln!(
                writer,
                "Lex error at line {line}, column {col}: Unterminated comment"
            )?;
        }
        LexErrorKind::IllegalByte(b) => {
            writeln!(
                writer,
                "Lex error at line {line}, column {col}: Illegal byte 0x{b:02X}"
            )?;
        }
    }
    writeln!(writer)
}
