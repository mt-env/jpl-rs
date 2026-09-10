use crate::{
    parser::ast::ParsedExpr,
    typechecker::{
        ast::{TypeError, TypedExpr},
        typecheck_ctx::TypecheckCtx,
    },
};

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
