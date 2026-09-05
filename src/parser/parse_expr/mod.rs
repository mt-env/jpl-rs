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
        TokenKind::FloatVal => parse_float(str, offset)?,
        TokenKind::IntVal => parse_int(str, offset)?,
        TokenKind::Variable => ExprKind::Var(str),
        TokenKind::LSquare => parse_array_literal(ctx)?,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };

    Ok(ParsedExpr::new(ctx, offset, kind))
}

fn parse_float<'src, 'ast>(
    str: &'src str,
    offset: usize,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Ok(parsed_float) = str.parse::<f64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidFloatLiteral(str),
        ));
    };
    Ok(ExprKind::Float(parsed_float))
}

fn parse_int<'src, 'ast>(
    str: &'src str,
    offset: usize,
) -> Result<ExprKind<'src, 'ast, ()>, ParseError<'src>> {
    let Ok(parsed_int) = str.parse::<i64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidIntLiteral(str),
        ));
    };
    Ok(ExprKind::Int(parsed_int))
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
