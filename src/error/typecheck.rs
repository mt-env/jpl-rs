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
    print!("Type error at line {line}, column {column}: ");
    match error {
        TypeErrorKind::ExpectType { expected, found } => {
            println!("Expected type '{expected}', found type '{found}'");
        }
        TypeErrorKind::ExpectTypes { expected, found } => {
            let expected_str = expected
                .iter()
                .map(|ty| format!("{ty}"))
                .collect::<Vec<_>>()
                .join(", ");
            println!("Expected one of types [{expected_str}], found type '{found}'");
        }
        TypeErrorKind::EmptyArrayLiteral => {
            println!("Empty array literal is not allowed");
        }
        TypeErrorKind::EmptyArrayLoopBindings => {
            println!("Empty array loop bindings are not allowed");
        }
        TypeErrorKind::EmptySumLoopBindings => {
            println!("Empty sum loop bindings are not allowed");
        }
        TypeErrorKind::UnknownIdentifier(name) => {
            println!("Unknown identifier '{name}'");
        }
        TypeErrorKind::UnknownStruct(name) => {
            println!("Unknown struct '{name}'");
        }
        TypeErrorKind::UnknownValue(name) => {
            println!("Unknown value '{name}'");
        }
        TypeErrorKind::UnknownFunction(name) => {
            println!("Unknown function '{name}'");
        }
        TypeErrorKind::DuplicateIdentifier(name) => {
            println!("Duplicate identifier '{name}'");
        }
        TypeErrorKind::DuplicateStructField {
            struct_name,
            field_name,
        } => {
            println!("Struct '{struct_name}' has duplicate field '{field_name}'");
        }
        TypeErrorKind::StructFieldCountMismatch {
            struct_name,
            expected,
            actual,
        } => {
            println!("Struct '{struct_name}' expects {expected} fields, but found {actual}");
        }
        TypeErrorKind::FunctionArgCountMismatch {
            function_name,
            expected,
            actual,
        } => {
            println!("Function '{function_name}' expects {expected} arguments, but found {actual}");
        }
        TypeErrorKind::DotOnNonStruct => {
            println!("Dot operator used on a non-struct type");
        }
        TypeErrorKind::UnknownStructField {
            struct_name,
            field_name,
        } => {
            println!("Struct '{struct_name}' has no field named '{field_name}'");
        }
        TypeErrorKind::ArrayIndexOnNonArray => {
            println!("Array index operator used on a non-array type");
        }
        TypeErrorKind::ArrayIndexDimensionMismatch { expected, actual } => {
            println!("Array index dimension mismatch: expected {expected}, found {actual}");
        }
        TypeErrorKind::ArrayLValueOnNonArray { rhs } => {
            println!("Array lvalue used on a non-array type: found type '{rhs}'");
        }
        TypeErrorKind::ArrayLValueDimensionMismatch { expected, actual } => {
            println!("Array lvalue dimension mismatch: expected {expected}, found {actual}");
        }
        TypeErrorKind::MissingReturn {
            fn_name,
            return_type,
        } => {
            println!(
                "Function '{fn_name}' is missing a return statement for return type '{return_type}'",
            );
        }
    }
}
