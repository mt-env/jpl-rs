use std::io::{self, Write};

use crate::{Spanned, error::CompilerError, typechecker::ast::TypeValue};

pub type TypeError<'src, 'ast> = Spanned<TypeErrorKind<'src, 'ast>>;

pub enum TypeErrorKind<'src, 'ast> {
    ExpectType {
        expected: &'ast TypeValue<'src, 'ast>,
        found: &'ast TypeValue<'src, 'ast>,
    },
    ExpectTypes {
        expected: &'ast [&'ast TypeValue<'src, 'ast>],
        found: &'ast TypeValue<'src, 'ast>,
    },
    EmptyArrayLiteral,
    EmptyArrayLoopBindings,
    EmptySumLoopBindings,
    UnknownIdentifier(&'src str),
    UnknownStruct(&'src str),
    UnknownValue(&'src str),
    UnknownFunction(&'src str),
    DuplicateIdentifier(&'src str),
    DuplicateStructField {
        struct_name: &'src str,
        field_name: &'src str,
    },
    StructFieldCountMismatch {
        struct_name: &'src str,
        expected: usize,
        actual: usize,
    },
    FunctionArgCountMismatch {
        function_name: &'src str,
        expected: usize,
        actual: usize,
    },
    DotOnNonStruct,
    UnknownStructField {
        struct_name: &'src str,
        field_name: &'src str,
    },
    ArrayIndexOnNonArray,
    ArrayIndexDimensionMismatch {
        expected: usize,
        actual: usize,
    },
    ArrayLValueOnNonArray {
        rhs: &'ast TypeValue<'src, 'ast>,
    },
    ArrayLValueDimensionMismatch {
        expected: usize,
        actual: usize,
    },
    MissingReturn {
        fn_name: &'src str,
        return_type: &'ast TypeValue<'src, 'ast>,
    },
}

impl CompilerError for TypeError<'_, '_> {
    fn print_diagnostic(&self, writer: &mut impl Write, line: usize, col: usize) -> io::Result<()> {
        write!(writer, "Type error at line {line}, column {col}: ")?;
        let error_str = match &self.value {
            TypeErrorKind::ExpectType { expected, found } => {
                &format!("Expected type '{expected}', found type '{found}'")
            }
            TypeErrorKind::ExpectTypes { expected, found } => {
                let expected_str = expected
                    .iter()
                    .map(|ty| format!("{ty}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                &format!("Expected one of types [{expected_str}], found type '{found}'")
            }
            TypeErrorKind::EmptyArrayLiteral => "Empty array literal is not allowed",
            TypeErrorKind::EmptyArrayLoopBindings => "Empty array loop bindings are not allowed",
            TypeErrorKind::EmptySumLoopBindings => "Empty sum loop bindings are not allowed",
            TypeErrorKind::UnknownIdentifier(name) => &format!("Unknown identifier '{name}'"),
            TypeErrorKind::UnknownStruct(name) => &format!("Unknown struct '{name}'"),
            TypeErrorKind::UnknownValue(name) => &format!("Unknown value '{name}'"),
            TypeErrorKind::UnknownFunction(name) => &format!("Unknown function '{name}'"),
            TypeErrorKind::DuplicateIdentifier(name) => &format!("Duplicate identifier '{name}'"),
            TypeErrorKind::DuplicateStructField {
                struct_name,
                field_name,
            } => &format!("Struct '{struct_name}' has duplicate field '{field_name}'"),
            TypeErrorKind::StructFieldCountMismatch {
                struct_name,
                expected,
                actual,
            } => &format!("Struct '{struct_name}' expects {expected} fields, but found {actual}"),
            TypeErrorKind::FunctionArgCountMismatch {
                function_name,
                expected,
                actual,
            } => &format!(
                "Function '{function_name}' expects {expected} arguments, but found {actual}"
            ),
            TypeErrorKind::DotOnNonStruct => "Dot operator used on a non-struct type",
            TypeErrorKind::UnknownStructField {
                struct_name,
                field_name,
            } => &format!("Struct '{struct_name}' has no field named '{field_name}'"),
            TypeErrorKind::ArrayIndexOnNonArray => "Array index operator used on a non-array type",
            TypeErrorKind::ArrayIndexDimensionMismatch { expected, actual } => {
                &format!("Array index dimension mismatch: expected {expected}, found {actual}")
            }
            TypeErrorKind::ArrayLValueOnNonArray { rhs } => {
                &format!("Array lvalue used on a non-array type: found type '{rhs}'")
            }
            TypeErrorKind::ArrayLValueDimensionMismatch { expected, actual } => {
                &format!("Array lvalue dimension mismatch: expected {expected}, found {actual}")
            }
            TypeErrorKind::MissingReturn {
                fn_name,
                return_type,
            } => &format!(
                "Function '{fn_name}' is missing a return statement for return type '{return_type}'"
            ),
        };
        writeln!(writer, "{error_str}")
    }

    fn get_offset(&self) -> usize {
        self.offset
    }
}
