use crate::{
    parser::ast::ParsedCmd,
    typechecker::ast::{TypeError, TypedProgram},
};

pub mod ast;
mod typecheck_cmd;
mod typecheck_ctx;
mod typecheck_expr;

pub fn typecheck<'src, 'old, 'new>(
    ast: Vec<ParsedCmd<'src, 'old>>,
    alloc: &'new bumpalo::Bump,
) -> Result<TypedProgram<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
