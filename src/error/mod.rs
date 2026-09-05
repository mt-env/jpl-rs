pub mod lex;
pub mod parse;

fn get_line_and_column(program: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    for (i, c) in program.chars().enumerate() {
        if i == pos {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn show_line_with_error(program: &str, pos: usize) {
    let (line, column) = get_line_and_column(program, pos);
    let line_start = program[..pos].rfind('\n').map_or(0, |i| i + 1);
    let line_end = program[pos..].find('\n').map_or(program.len(), |i| pos + i);
    let line_content = &program[line_start..line_end];
    println!("{line} | {line_content}");
    println!(
        "{} | {}^",
        " ".repeat(line.to_string().len()),
        " ".repeat(column - 1)
    );
}
