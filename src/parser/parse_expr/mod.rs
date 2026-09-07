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
        TokenKind::Void,
    ])?;
    let kind = match kind {
        TokenKind::True => ExprKind::Bool(true),
        TokenKind::False => ExprKind::Bool(false),
        TokenKind::FloatVal => parse_float(str, offset)?,
        TokenKind::IntVal => parse_int(str, offset)?,
        TokenKind::Variable => {
            // parse struct literal expr
            if ctx.peek_is(TokenKind::LCurly) {
                ctx.expect(TokenKind::LCurly)?;
                let mut fields = Vec::new();
                if !ctx.peek_is(TokenKind::RCurly) {
                    loop {
                        fields.push(parse_expr(ctx)?);
                        if !ctx.peek_is(TokenKind::Comma) {
                            break;
                        }
                        ctx.expect(TokenKind::Comma)?;
                    }
                }
                ctx.expect(TokenKind::RCurly)?;
                return Ok(ParsedExpr::new(
                    ctx,
                    offset,
                    ExprKind::StructLiteral(str, fields),
                ));
            }

            // parse call expr
            if ctx.peek_is(TokenKind::LParen) {
                ctx.expect(TokenKind::LParen)?;
                let mut args = Vec::new();
                if !ctx.peek_is(TokenKind::RParen) {
                    loop {
                        args.push(parse_expr(ctx)?);
                        if !ctx.peek_is(TokenKind::Comma) {
                            break;
                        }
                        ctx.expect(TokenKind::Comma)?;
                    }
                }
                ctx.expect(TokenKind::RParen)?;
                return Ok(ParsedExpr::new(ctx, offset, ExprKind::Call(str, args)));
            }

            ExprKind::Var(str)
        }
        TokenKind::Void => ExprKind::Void,
        TokenKind::LSquare => parse_array_literal(ctx)?,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };

    let expr = ParsedExpr::new(ctx, offset, kind);

    // parse dot expr
    if ctx.peek_is(TokenKind::Dot) {
        ctx.expect(TokenKind::Dot)?;
        let Token { str, .. } = ctx.expect(TokenKind::Variable)?;
        let dot_expr = ExprKind::Dot(expr, str);
        return Ok(ParsedExpr::new(ctx, offset, dot_expr));
    }

    // parse array index expr
    if ctx.peek_is(TokenKind::LSquare) {
        ctx.expect(TokenKind::LSquare)?;
        let mut indices = Vec::new();
        if !ctx.peek_is(TokenKind::RSquare) {
            loop {
                let index_expr = parse_expr(ctx)?;
                indices.push(index_expr);
                if !ctx.peek_is(TokenKind::Comma) {
                    break;
                }
                ctx.expect(TokenKind::Comma)?;
            }
        }
        ctx.expect(TokenKind::RSquare)?;
        let array_index_expr = ExprKind::ArrayIndex(expr, ctx.alloc(indices));
        return Ok(ParsedExpr::new(ctx, offset, array_index_expr));
    }

    Ok(expr)
}

fn parse_int<'ast>(str: &str, offset: usize) -> Result<ExprKind<'_, 'ast, ()>, ParseError<'_>> {
    let Ok(parsed_int) = str.parse::<i64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidIntLiteral(str),
        ));
    };
    Ok(ExprKind::Int(parsed_int))
}

fn parse_float<'ast>(str: &str, offset: usize) -> Result<ExprKind<'_, 'ast, ()>, ParseError<'_>> {
    let Ok(parsed_float) = str.parse::<f64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidFloatLiteral(str),
        ));
    };
    if !parsed_float.is_finite() {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidFloatLiteral(str),
        ));
    }
    Ok(ExprKind::Float(parsed_float))
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
