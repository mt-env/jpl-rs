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

    if !ctx.peek_is(TokenKind::LSquare) {
        return Ok(ParsedLValue::new(ctx, offset, LValue::Var(str)));
    }

    // parse array dimension bindings if necessary
    ctx.expect(TokenKind::LSquare)?;
    let mut vars = Vec::new();
    if !ctx.peek_is(TokenKind::RSquare) {
        loop {
            let Token { str, .. } = ctx.expect(TokenKind::Variable)?;
            vars.push(str);
            if !ctx.peek_is(TokenKind::Comma) {
                break;
            }
            ctx.expect(TokenKind::Comma)?;
        }
    }
    ctx.expect(TokenKind::RSquare)?;
    Ok(ParsedLValue::new(ctx, offset, LValue::Array(str, vars)))
}
