use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{LValue, ParseError, ParsedLValue},
    },
};

pub(super) fn parse_lvalue<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedLValue<'src>, ParseError<'src>> {
    let Token {
        str,
        offset,
        kind: _,
    } = ctx.expect(TokenKind::Variable)?;

    Ok(ParsedLValue::new(ctx, offset, LValue::Var(str)))
}
