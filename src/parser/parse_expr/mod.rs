use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{ExprKind, ParsedExpr},
    },
};

pub(super) fn parse_expr<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ()> {
    let Some(Token { str, offset, kind }) = ctx.peek() else {
        return Err(());
    };
    let kind = match kind {
        TokenKind::True => ExprKind::Bool(true),
        TokenKind::False => ExprKind::Bool(false),
        TokenKind::FloatVal => parse_float(ctx)?,
        TokenKind::IntVal => parse_int(ctx)?,
        TokenKind::Variable => ExprKind::Var(str),
        TokenKind::LSquare => parse_array_literal(ctx)?,
        _ => todo!(),
    };

    Ok(ParsedExpr::new(ctx, offset, kind))
}

fn parse_float<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ()> {
    let Token { str, .. } = ctx.expect(TokenKind::FloatVal)?;
    let parsed_float = match str.parse::<f64>() {
        Ok(f) => f,
        Err(_) => return Err(()),
    };
    Ok(ExprKind::Float(parsed_float))
}

fn parse_int<'src, 'ast>(ctx: &mut ParserCtx<'src, 'ast>) -> Result<ExprKind<'src, 'ast, ()>, ()> {
    let Token { str, .. } = ctx.expect(TokenKind::IntVal)?;
    let parsed_int = match str.parse::<i64>() {
        Ok(i) => i,
        Err(_) => return Err(()),
    };
    Ok(ExprKind::Int(parsed_int))
}

fn parse_array_literal<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ()> {
    ctx.expect(TokenKind::LSquare)?;
    let mut elements = Vec::new();
    loop {
        if ctx.peek_is(TokenKind::RSquare) {
            break;
        }
        let element = parse_expr(ctx)?;
        elements.push(element);
        if ctx.peek_is(TokenKind::Comma) {
            ctx.expect(TokenKind::Comma)?;
        } else {
            break;
        }
    }
    return Ok(ExprKind::ArrayLiteral(elements));
}
