use crate::{
    parser::ast::{Cmd, ParsedCmd, ParsedExpr, ParsedType},
    typechecker::{
        ast::{TypeError, TypedCmd},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr, typecheck_type,
    },
};

pub(super) fn typecheck_cmd<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    cmd: &'old ParsedCmd<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let loc = cmd.offset;
    match &cmd.value {
        Cmd::Show(expr) => typecheck_show(ctx, loc, expr),
        Cmd::Struct { name, fields } => typecheck_struct(ctx, name, fields),
        _ => todo!(),
    }
}

fn typecheck_show<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Show(typed_expr)))
}

fn typecheck_struct<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    name: &'old str,
    fields: &Vec<(&'src str, &'old ParsedType<'src, 'old>)>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
