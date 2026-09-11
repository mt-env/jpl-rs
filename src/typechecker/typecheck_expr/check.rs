use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypeErrorKind, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr,
    },
};

pub(super) fn check<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}

pub(super) fn check_num<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    if !matches!(typed_expr.value.ann, TypeValue::Int | TypeValue::Float) {
        return Err(TypeError {
            offset: typed_expr.offset,
            value: TypeErrorKind::ExpectTypes {
                expected: &[&TypeValue::Int, &TypeValue::Float],
                found: typed_expr.value.ann,
            },
        });
    }

    Ok(typed_expr)
}
