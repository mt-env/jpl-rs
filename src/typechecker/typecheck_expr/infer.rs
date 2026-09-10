use crate::{
    parser::ast::{Expr, ExprKind, ParsedExpr},
    typechecker::{
        ast::{TypeError, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
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
        ExprKind::If(_, _, _) => todo!(),
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
    TypedExpr::new(ctx, offset, ExprKind::Int(val), TypeValue::Int)
}

fn infer_float<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: f64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Float(val), TypeValue::Float)
}

fn infer_bool<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: bool,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Bool(val), TypeValue::Bool)
}

fn infer_void<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Void, TypeValue::Void)
}
