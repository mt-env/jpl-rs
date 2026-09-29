use std::io::Write;

pub mod lex;
pub mod parse;
pub mod typecheck;

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
) -> std::io::Result<()> {
    let (line, column) = get_line_and_column(program, pos);
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
