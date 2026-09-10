use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
    },
};

pub(super) fn check<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
