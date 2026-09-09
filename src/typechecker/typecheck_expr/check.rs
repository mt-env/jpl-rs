use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypeValue, TypedCmd},
        typecheck_ctx::TypecheckCtx,
    },
};

pub(super) fn check<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
