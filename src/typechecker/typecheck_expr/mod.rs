use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
    },
};

mod check;
mod infer;

pub(super) fn check<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    // can't chain `pub(super) use`, lame
    check::check(ctx, expr, expected)
}

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    infer::infer(ctx, expr)
}

pub(super) fn check_num<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    check::check_many(ctx, expr, &[&TypeValue::Int, &TypeValue::Float])
}

pub(super) fn check_primitive<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    check::check_many(
        ctx,
        expr,
        &[&TypeValue::Int, &TypeValue::Float, &TypeValue::Bool],
    )
}
