use std::io::{self, Write};

use crate::{
    error::CompilerError,
    lexer::token::{LexError, LexErrorKind},
};

impl CompilerError for LexError {
    fn print_diagnostic(&self, writer: &mut impl Write, line: usize, col: usize) -> io::Result<()> {
        write!(writer, "Lex error at line {line}, column {col}: ")?;
        match &self.value {
            LexErrorKind::UnterminatedString => {
                writeln!(writer, "Unterminated string literal")?;
            }
            LexErrorKind::IllegalCharacter(c) => {
                writeln!(writer, "Illegal character '{c}'")?;
            }
            LexErrorKind::UnterminatedComment => {
                writeln!(writer, "Unterminated comment")?;
            }
            LexErrorKind::IllegalByte(b) => {
                writeln!(writer, "Illegal byte 0x{b:02X}")?;
            }
        }
        Ok(())
    }

    fn get_offset(&self) -> usize {
        self.offset
    }
}
