use crate::parser::{
    ParserCtx,
    ast::{ParseError, ParsedExpr},
};

mod parse_primary;

pub(super) fn parse_expr<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    todo!()
}
