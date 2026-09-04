use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{ExprKind, ParseError, ParseErrorKind, ParsedExpr},
    },
};

pub(super) fn parse_expr<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    let Some(Token { offset, kind, .. }) = ctx.peek() else {
        todo!()
    };
    let kind = match kind {
        TokenKind::True => parse_bool(ctx)?,
        TokenKind::False => parse_bool(ctx)?,
        TokenKind::FloatVal => parse_float(ctx)?,
        TokenKind::IntVal => parse_int(ctx)?,
        TokenKind::Variable => parse_var(ctx)?,
        TokenKind::LSquare => parse_array_literal(ctx)?,
        _ => todo!(),
    };

    Ok(ParsedExpr::new(ctx, offset, kind))
}

fn parse_bool<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Token { kind, .. } = ctx.expect_many(&[TokenKind::True, TokenKind::False])?;
    let value = match kind {
        TokenKind::True => true,
        TokenKind::False => false,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };
    Ok(ExprKind::Bool(value))
}

fn parse_float<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Token { offset, str, .. } = ctx.expect(TokenKind::FloatVal)?;
    let Ok(parsed_float) = str.parse::<f64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidFloatLiteral(str),
        ));
    };
    Ok(ExprKind::Float(parsed_float))
}

fn parse_int<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Token { offset, str, .. } = ctx.expect(TokenKind::IntVal)?;
    let Ok(parsed_int) = str.parse::<i64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidIntLiteral(str),
        ));
    };
    Ok(ExprKind::Int(parsed_int))
}

fn parse_var<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Token { str, .. } = ctx.expect(TokenKind::Variable)?;
    Ok(ExprKind::Var(str))
}

fn parse_array_literal<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
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
    ctx.expect(TokenKind::RSquare)?;
    Ok(ExprKind::ArrayLiteral(elements))
}
