use crate::{
    Spanned,
    typechecker::ast::{TypeError, TypeErrorKind},
};

pub fn print_type_error(
    Spanned {
        offset,
        value: error,
    }: TypeError,
    program: &str,
) {
    let (line, column) = super::get_line_and_column(program, offset);
    super::show_line_with_error(program, offset);
    match error {
        TypeErrorKind::ExpectType { expected, found } => {
            println!(
                "Type error at line {line}, column {column}: Expected type '{expected}', found type '{found}'",
            );
        }
        TypeErrorKind::ExpectTypes { expected, found } => {
            let expected_str = expected
                .iter()
                .map(|ty| format!("{ty}"))
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "Type error at line {line}, column {column}: Expected one of types [{expected_str}], found type '{found}'",
            );
        }
        TypeErrorKind::EmptyArrayLiteral => {
            println!(
                "Type error at line {line}, column {column}: Empty array literal is not allowed",
            );
        }
        TypeErrorKind::UnknownStruct(name) => {
            println!("Type error at line {line}, column {column}: Unknown struct '{name}'");
        }
        TypeErrorKind::StructFieldCountMismatch {
            struct_name,
            expected,
            actual,
        } => {
            println!(
                "Type error at line {line}, column {column}: Struct '{struct_name}' expects {expected} fields, but found {actual}",
            );
        }
        TypeErrorKind::DotOnNonStruct => {
            println!(
                "Type error at line {line}, column {column}: Dot operator used on a non-struct type",
            );
        }
        TypeErrorKind::UnknownStructField {
            struct_name,
            field_name,
        } => {
            println!(
                "Type error at line {line}, column {column}: Struct '{struct_name}' has no field named '{field_name}'",
            );
        }
        TypeErrorKind::ArrayIndexOnNonArray => {
            println!(
                "Type error at line {line}, column {column}: Array index operator used on a non-array type",
            );
        }
        TypeErrorKind::ArrayIndexDimensionMismatch { expected, actual } => {
            println!(
                "Type error at line {line}, column {column}: Array index dimension mismatch: expected {expected}, found {actual}",
            );
        }
    }
}
