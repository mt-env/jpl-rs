use bumpalo::Bump;

use crate::{
    parser::ast::ParsedCmd,
    typechecker::ast::{TypeError, TypedProgram},
};

pub mod ast;
mod typecheck_cmd;
mod typecheck_ctx;
mod typecheck_expr;

pub fn typecheck<'src, 'old, 'new>(
    alloc: &'new Bump,
    ast: Vec<&'old ParsedCmd<'src, 'old>>,
) -> Result<TypedProgram<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
