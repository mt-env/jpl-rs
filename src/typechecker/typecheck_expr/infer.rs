use crate::{
    parser::ast::ParsedExpr,
    typechecker::ast::{TypeError, TypedCmd},
};

pub(super) fn infer<'src, 'old, 'new>(
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
