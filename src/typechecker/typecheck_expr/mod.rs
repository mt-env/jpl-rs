use crate::{
    parser::ast::ParsedExpr,
    typechecker::ast::{TypeError, TypeValue, TypedCmd},
};

mod check;
mod infer;

pub(super) fn check<'src, 'old, 'new>(
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    // can't chain `pub(super) use`, lame
    check::check(expr, expected)
}

pub(super) fn infer<'src, 'old, 'new>(
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    infer::infer(expr)
}
