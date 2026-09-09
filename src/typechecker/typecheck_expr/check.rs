use crate::{
    parser::ast::ParsedExpr,
    typechecker::ast::{TypeError, TypeValue, TypedCmd},
};

pub(super) fn check<'src, 'old, 'new>(
    expr: &'old ParsedExpr<'src, 'old>,
    expected: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
