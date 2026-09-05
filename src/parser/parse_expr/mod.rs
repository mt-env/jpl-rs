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
    let Token { offset, kind, str } = ctx.expect_many(&[
        TokenKind::True,
        TokenKind::False,
        TokenKind::FloatVal,
        TokenKind::IntVal,
        TokenKind::Variable,
        TokenKind::LSquare,
    ])?;
    let kind = match kind {
        TokenKind::True => ExprKind::Bool(true),
        TokenKind::False => ExprKind::Bool(false),
        TokenKind::FloatVal => parse_numeric(
            str,
            offset,
            ExprKind::Float,
            ParseErrorKind::InvalidFloatLiteral,
        )?,
        TokenKind::IntVal => parse_numeric(
            str,
            offset,
            ExprKind::Int,
            ParseErrorKind::InvalidIntLiteral,
        )?,
        TokenKind::Variable => ExprKind::Var(str),
        TokenKind::LSquare => parse_array_literal(ctx)?,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };

    Ok(ParsedExpr::new(ctx, offset, kind))
}

fn parse_numeric<'src, 'ast, T>(
    str: &'src str,
    offset: usize,
    kind: impl Fn(T) -> ExprKind<'src, 'ast, ()>,
    error: impl Fn(&'src str) -> ParseErrorKind,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>>
where
    T: std::str::FromStr,
{
    let Ok(parsed_numeric) = str.parse::<T>() else {
        return Err(ParseError::new(offset, error(str)));
    };
    Ok(kind(parsed_numeric))
}

fn parse_array_literal<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
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
