use std::io::{self, Write};

use crate::{Spanned, error::CompilerError, lexer::token::TokenKind};

pub enum ParseErrorKind<'src> {
    InvalidIntLiteral(&'src str),
    InvalidFloatLiteral(&'src str),
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
        value: &'src str,
    },
}

pub type ParseError<'src> = Spanned<ParseErrorKind<'src>>;

impl CompilerError for ParseError<'_> {
    fn print_diagnostic(&self, writer: &mut impl Write, line: usize, col: usize) -> io::Result<()> {
        write!(writer, "Parse error at line {line}, column {col}: ")?;
        match &self.value {
            ParseErrorKind::InvalidIntLiteral(int) => {
                writeln!(writer, "Invalid integer literal '{int}'")?;
            }
            ParseErrorKind::InvalidFloatLiteral(float) => {
                writeln!(writer, "Invalid float literal '{float}'")?;
            }
            ParseErrorKind::UnexpectedToken {
                expected,
                found,
                value,
            } => {
                let expected_str = expected
                    .iter()
                    .map(|kind| format!("{kind}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    writer,
                    "Unexpected token '{value}' of kind '{found}', expected one of: {expected_str}"
                )?;
            }
        }
        Ok(())
    }

    fn get_offset(&self) -> usize {
        self.offset
    }
}
