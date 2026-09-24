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
        TypeErrorKind::EmptyArrayLoopBindings => {
            println!(
                "Type error at line {line}, column {column}: Empty array loop bindings are not allowed",
            );
        }
        TypeErrorKind::EmptySumLoopBindings => {
            println!(
                "Type error at line {line}, column {column}: Empty sum loop bindings are not allowed",
            );
        }
        TypeErrorKind::UnknownIdentifier(name) => {
            println!("Type error at line {line}, column {column}: Unknown identifier '{name}'");
        }
        TypeErrorKind::UnknownStruct(name) => {
            println!("Type error at line {line}, column {column}: Unknown struct '{name}'");
        }
        TypeErrorKind::UnknownValue(name) => {
            println!("Type error at line {line}, column {column}: Unknown value '{name}'");
        }
        TypeErrorKind::UnknownFunction(name) => {
            println!("Type error at line {line}, column {column}: Unknown function '{name}'");
        }
        TypeErrorKind::DuplicateIdentifier(name) => {
            println!("Type error at line {line}, column {column}: Duplicate identifier '{name}'");
        }
        TypeErrorKind::DuplicateStructField {
            struct_name,
            field_name,
        } => {
            println!(
                "Type error at line {line}, column {column}: Struct '{struct_name}' has duplicate field '{field_name}'",
            );
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
        TypeErrorKind::FunctionArgCountMismatch {
            function_name,
            expected,
            actual,
        } => {
            println!(
                "Type error at line {line}, column {column}: Function '{function_name}' expects {expected} arguments, but found {actual}",
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
        TypeErrorKind::ArrayLValueOnNonArray { rhs } => {
            println!(
                "Type error at line {line}, column {column}: Array lvalue used on a non-array type: found type '{rhs}'",
            );
        }
        TypeErrorKind::ArrayLValueDimensionMismatch { expected, actual } => {
            println!(
                "Type error at line {line}, column {column}: Array lvalue dimension mismatch: expected {expected}, found {actual}",
            );
        }
        TypeErrorKind::MissingReturn {
            fn_name,
            return_type,
        } => {
            println!(
                "Type error at line {line}, column {column}: Function '{fn_name}' is missing a return statement for return type '{return_type}'",
            );
        }
    }
}
