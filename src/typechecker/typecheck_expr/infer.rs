use crate::{
    parser::ast::{ExprKind, ParsedExpr},
    typechecker::{
        ast::{TypeError, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr,
    },
};

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let loc = expr.offset;
    match &expr.value.kind {
        ExprKind::Int(val) => Ok(infer_int(ctx, loc, *val)),
        ExprKind::Float(val) => Ok(infer_float(ctx, loc, *val)),
        ExprKind::Bool(val) => Ok(infer_bool(ctx, loc, *val)),
        ExprKind::Var(_) => todo!(),
        ExprKind::Void => Ok(infer_void(ctx, loc)),
        ExprKind::ArrayLiteral(_) => todo!(),
        ExprKind::StructLiteral(_, _) => todo!(),
        ExprKind::Dot(_, _) => todo!(),
        ExprKind::ArrayIndex(_, _) => todo!(),
        ExprKind::Call(_, _) => todo!(),
        ExprKind::If(cond, thenb, elseb) => infer_if(ctx, loc, cond, thenb, elseb),
        ExprKind::ArrayLoop(_, _) => todo!(),
        ExprKind::SumLoop(_, _) => todo!(),
        ExprKind::Unary(_, _) => todo!(),
        ExprKind::Binary(_, _, _) => todo!(),
    }
}

fn infer_int<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: i64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Int(val), &TypeValue::Int)
}

fn infer_float<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: f64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Float(val), &TypeValue::Float)
}

fn infer_bool<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: bool,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Bool(val), &TypeValue::Bool)
}

fn infer_void<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Void, &TypeValue::Void)
}

fn infer_if<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    cond: &'old ParsedExpr<'src, 'old>,
    thenb: &'old ParsedExpr<'src, 'old>,
    elseb: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_cond = typecheck_expr::check(ctx, cond, &TypeValue::Bool)?;
    let typed_thenb = infer(ctx, thenb)?;
    let ternary_type = typed_thenb.value.ann;
    let typed_elseb = typecheck_expr::check(ctx, elseb, ternary_type)?;
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::If(typed_cond, typed_thenb, typed_elseb),
        ternary_type,
    ))
}
