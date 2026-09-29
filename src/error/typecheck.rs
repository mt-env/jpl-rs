use std::io::{self, BufWriter, Write};

use crate::{
    Spanned,
    typechecker::ast::{TypeError, TypeErrorKind},
};

pub fn print_type_error(
    Spanned {
        offset,
        value: error,
    }: TypeError,
    program: &[u8],
) -> io::Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    let (line, column) = super::get_line_and_column(program, offset);
    super::show_line_with_error(&mut writer, program, offset)?;
    print!("Type error at line {line}, column {column}: ");
    match error {
        TypeErrorKind::ExpectType { expected, found } => {
            writeln!(writer, "Expected type '{expected}', found type '{found}'")
        }
        TypeErrorKind::ExpectTypes { expected, found } => {
            let expected_str = expected
                .iter()
                .map(|ty| format!("{ty}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                writer,
                "Expected one of types [{expected_str}], found type '{found}'"
            )
        }
        TypeErrorKind::EmptyArrayLiteral => {
            writeln!(writer, "Empty array literal is not allowed")
        }
        TypeErrorKind::EmptyArrayLoopBindings => {
            writeln!(writer, "Empty array loop bindings are not allowed")
        }
        TypeErrorKind::EmptySumLoopBindings => {
            writeln!(writer, "Empty sum loop bindings are not allowed")
        }
        TypeErrorKind::UnknownIdentifier(name) => {
            writeln!(writer, "Unknown identifier '{name}'")
        }
        TypeErrorKind::UnknownStruct(name) => {
            writeln!(writer, "Unknown struct '{name}'")
        }
        TypeErrorKind::UnknownValue(name) => {
            writeln!(writer, "Unknown value '{name}'")
        }
        TypeErrorKind::UnknownFunction(name) => {
            writeln!(writer, "Unknown function '{name}'")
        }
        TypeErrorKind::DuplicateIdentifier(name) => {
            writeln!(writer, "Duplicate identifier '{name}'")
        }
        TypeErrorKind::DuplicateStructField {
            struct_name,
            field_name,
        } => {
            writeln!(
                writer,
                "Struct '{struct_name}' has duplicate field '{field_name}'"
            )
        }
        TypeErrorKind::StructFieldCountMismatch {
            struct_name,
            expected,
            actual,
        } => {
            writeln!(
                writer,
                "Struct '{struct_name}' expects {expected} fields, but found {actual}"
            )
        }
        TypeErrorKind::FunctionArgCountMismatch {
            function_name,
            expected,
            actual,
        } => {
            writeln!(
                writer,
                "Function '{function_name}' expects {expected} arguments, but found {actual}"
            )
        }
        TypeErrorKind::DotOnNonStruct => {
            writeln!(writer, "Dot operator used on a non-struct type")
        }
        TypeErrorKind::UnknownStructField {
            struct_name,
            field_name,
        } => {
            writeln!(
                writer,
                "Struct '{struct_name}' has no field named '{field_name}'"
            )
        }
        TypeErrorKind::ArrayIndexOnNonArray => {
            writeln!(writer, "Array index operator used on a non-array type")
        }
        TypeErrorKind::ArrayIndexDimensionMismatch { expected, actual } => {
            writeln!(
                writer,
                "Array index dimension mismatch: expected {expected}, found {actual}"
            )
        }
        TypeErrorKind::ArrayLValueOnNonArray { rhs } => {
            writeln!(
                writer,
                "Array lvalue used on a non-array type: found type '{rhs}'"
            )
        }
        TypeErrorKind::ArrayLValueDimensionMismatch { expected, actual } => {
            writeln!(
                writer,
                "Array lvalue dimension mismatch: expected {expected}, found {actual}"
            )
        }
        TypeErrorKind::MissingReturn {
            fn_name,
            return_type,
        } => {
            writeln!(
                writer,
                "Function '{fn_name}' is missing a return statement for return type '{return_type}'",
            )
        }
    }
}
