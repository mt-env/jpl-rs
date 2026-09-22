use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypeErrorKind, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr,
    },
};

pub(super) fn check<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    if typed_expr.value.ann == expected {
        Ok(typed_expr)
    } else {
        Err(TypeError {
            offset: typed_expr.offset,
            value: TypeErrorKind::ExpectType {
                expected,
                found: typed_expr.value.ann,
            },
        })
    }
}

pub(super) fn check_many<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new [&'new TypeValue<'src, 'new>],
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    for expected_type in expected {
        if typed_expr.value.ann == *expected_type {
            return Ok(typed_expr);
        }
    }
    Err(TypeError {
        offset: typed_expr.offset,
        value: TypeErrorKind::ExpectTypes {
            expected,
            found: typed_expr.value.ann,
        },
    })
}
