use crate::{Spanned, typechecker::ast::TypeError};

pub fn print_type_error(
    Spanned {
        offset,
        value: error,
    }: TypeError,
    program: &str,
) {
    todo!()
}
