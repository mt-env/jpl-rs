use bumpalo::Bump;

use crate::{
    parser::ast::ParsedCmd,
    typechecker::{
        ast::{TypeError, TypedProgram},
        typecheck_ctx::TypecheckCtx,
    },
};

pub mod ast;
mod typecheck_cmd;
mod typecheck_ctx;
mod typecheck_expr;
mod typecheck_stmt;
mod typecheck_type;

pub fn typecheck<'src, 'old, 'new>(
    alloc: &'new Bump,
    ast: Vec<&'old ParsedCmd<'src, 'old>>,
) -> Result<TypedProgram<'src, 'new>, TypeError<'src, 'new>> {
    let mut typed_ast = Vec::new();
    let mut ctx = TypecheckCtx::new(alloc);
    for cmd in ast {
        typed_ast.push(typecheck_cmd::typecheck_cmd(&mut ctx, cmd)?)
    }
    Ok(typed_ast)
}
