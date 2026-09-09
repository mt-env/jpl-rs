use crate::{
    Spanned,
    parser::ast::{ParseError, ParseErrorKind},
};

pub fn print_parse_error(
    Spanned {
        offset,
        value: error,
    }: ParseError,
    program: &str,
) {
    let (line, column) = super::get_line_and_column(program, offset);
    super::show_line_with_error(program, offset);
    match error {
        ParseErrorKind::InvalidIntLiteral(src) => {
            println!(
                "Parse error at line {line}, column {column}: Invalid integer literal '{src}'",
            );
        }
        ParseErrorKind::InvalidFloatLiteral(src) => {
            println!("Parse error at line {line}, column {column}: Invalid float literal '{src}'");
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
            println!(
                "Parse error at line {line}, column {column}: Unexpected token '{value}' of kind '{found}', expected one of: {expected_str}",
            );
        }
    }
}
