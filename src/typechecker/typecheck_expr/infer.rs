use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypedCmd},
        typecheck_ctx::TypecheckCtx,
    },
};

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
