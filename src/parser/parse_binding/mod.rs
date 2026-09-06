use crate::{
    lexer::token::TokenKind,
    parser::{
        ParserCtx,
        ast::{Binding, ParseError, ParsedBinding},
        parse_lvalue, parse_type,
    },
};

pub(super) fn parse_binding<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedBinding<'src, 'ast>, ParseError<'src>> {
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    let offset = lvalue.offset;
    ctx.expect(TokenKind::Colon)?;
    let ty = parse_type::parse_type(ctx)?;

    Ok(ParsedBinding::new(ctx, offset, Binding { lvalue, ty }))
}
