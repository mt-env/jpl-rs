use crate::{parser::ast::ParsedCmd, typechecker::ast::TypedProgram};

pub mod ast;

pub fn typecheck<'src, 'old, 'new>(ast: Vec<ParsedCmd<'src, 'old>>) -> TypedProgram<'src, 'new> {
    todo!()
}
