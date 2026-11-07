use std::io::{self, BufWriter, Write};

mod lex;
mod parse;
mod typecheck;

pub use lex::LexError;
pub use lex::LexErrorKind;
pub use parse::ParseError;
pub use parse::ParseErrorKind;
pub use typecheck::TypeError;
pub use typecheck::TypeErrorKind;

pub fn print_lex_errors(errors: &[LexError], program: &[u8]) -> io::Result<()> {
    print_errors(&mut BufWriter::new(io::stdout().lock()), errors, program)
}

pub fn print_parse_error(error: &ParseError, program: &[u8]) -> io::Result<()> {
    print_error(&mut BufWriter::new(io::stdout().lock()), error, program)
}

pub fn print_type_error(error: &TypeError, program: &[u8]) -> io::Result<()> {
    print_error(&mut BufWriter::new(io::stdout().lock()), error, program)
}

trait CompilerError {
    fn print_diagnostic(&self, writer: &mut impl Write, line: usize, col: usize) -> io::Result<()>;
    fn get_offset(&self) -> usize;
}

fn print_errors(
    writer: &mut impl Write,
    errors: &[impl CompilerError],
    program: &[u8],
) -> io::Result<()> {
    for error in errors {
        print_error(writer, error, program)?;
    }
    writer.flush()
}

fn print_error(
    writer: &mut impl Write,
    error: &impl CompilerError,
    program: &[u8],
) -> io::Result<()> {
    let offset = error.get_offset();
    let (line, column) = get_line_and_column(program, offset);
    show_line_with_error(writer, program, offset, line, column)?;
    error.print_diagnostic(writer, line, column)?;
    writeln!(writer)?;
    writer.flush()
}

fn get_line_and_column(program: &[u8], pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    for (i, c) in program.iter().enumerate() {
        if i == pos {
            break;
        }
        if *c == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn show_line_with_error(
    writer: &mut impl Write,
    program: &[u8],
    pos: usize,
    line: usize,
    column: usize,
) -> std::io::Result<()> {
    let line_start = program[..pos]
        .iter()
        .rposition(|&c| c == b'\n')
        .map_or(0, |i| i + 1);
    let line_end = program[pos..]
        .iter()
        .position(|&c| c == b'\n')
        .map_or(program.len(), |i| pos + i);
    let line_content =
        String::from_utf8_lossy(program.get(line_start..line_end).unwrap_or_default()); // TODO
    writeln!(writer, "{line} | {line_content}")?;
    writeln!(
        writer,
        "{} | {}^",
        " ".repeat(line.to_string().len()),
        " ".repeat(column - 1)
    )
}
