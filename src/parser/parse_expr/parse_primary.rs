use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{ExprKind, LoopIterVar, ParseError, ParseErrorKind, ParsedExpr, ParsedLoopIterVar},
        parse_expr::parse_expr,
    },
};

pub(super) fn parse_primary<'src, 'ast>(
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
        TokenKind::LParen,
        TokenKind::If,
        TokenKind::Array,
        TokenKind::Sum,
    ])?;
    let mut expr = match kind {
        TokenKind::True => ParsedExpr::new(ctx, offset, ExprKind::Bool(true)),
        TokenKind::False => ParsedExpr::new(ctx, offset, ExprKind::Bool(false)),
        TokenKind::FloatVal => parse_float(ctx, str, offset)?,
        TokenKind::IntVal => parse_int(ctx, str, offset)?,
        TokenKind::Variable => {
            // parse struct literal expr
            if ctx.peek_is(TokenKind::LCurly) {
                parse_struct_literal(ctx, offset, str)?
            }
            // parse call expr
            else if ctx.peek_is(TokenKind::LParen) {
                parse_call(ctx, offset, str)?
            } else {
                ParsedExpr::new(ctx, offset, ExprKind::Var(str))
            }
        }
        TokenKind::Void => ParsedExpr::new(ctx, offset, ExprKind::Void),
        TokenKind::LSquare => parse_array_literal(ctx, offset)?,
        TokenKind::LParen => {
            let expr = parse_expr(ctx)?;
            ctx.expect(TokenKind::RParen)?;
            expr
        }
        TokenKind::If => parse_if(ctx, offset)?,
        TokenKind::Array => parse_loop_expr(ctx, offset, ExprKind::ArrayLoop)?,
        TokenKind::Sum => parse_loop_expr(ctx, offset, ExprKind::SumLoop)?,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };

    loop {
        // parse dot expr
        if ctx.peek_is(TokenKind::Dot) {
            expr = parse_dot(ctx, expr)?;
            continue;
        }
        // parse array index expr
        if ctx.peek_is(TokenKind::LSquare) {
            expr = parse_array_index(ctx, expr)?;
            continue;
        }
        break;
    }
    Ok(expr)
}

fn parse_int<'src, 'ast>(
    ctx: &ParserCtx<'src, 'ast>,
    str: &'src str,
    offset: usize,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    let Ok(parsed_int) = str.parse::<i64>() else {
        return Err(ParseError::new(
            offset,
            ParseErrorKind::InvalidIntLiteral(str),
        ));
    };
    Ok(ParsedExpr::new(ctx, offset, ExprKind::Int(parsed_int)))
}

fn parse_float<'src, 'ast>(
    ctx: &ParserCtx<'src, 'ast>,
    str: &'src str,
    offset: usize,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
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
    Ok(ParsedExpr::new(ctx, offset, ExprKind::Float(parsed_float)))
}

fn parse_array_literal<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
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
    Ok(ParsedExpr::new(
        ctx,
        offset,
        ExprKind::ArrayLiteral(elements),
    ))
}

fn parse_dot<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    inner: &'ast ParsedExpr<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    ctx.expect(TokenKind::Dot)?;
    let Token { str, .. } = ctx.expect(TokenKind::Variable)?;
    let dot_expr = ExprKind::Dot(inner, str);
    Ok(ParsedExpr::new(ctx, inner.offset, dot_expr))
}

fn parse_array_index<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    inner: &'ast ParsedExpr<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
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
    let array_index_expr = ExprKind::ArrayIndex(inner, ctx.alloc(indices));
    Ok(ParsedExpr::new(ctx, inner.offset, array_index_expr))
}

fn parse_struct_literal<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
    struct_name: &'src str,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    ctx.expect(TokenKind::LCurly)?;
    let mut fields = Vec::new();
    if !ctx.peek_is(TokenKind::RCurly) {
        loop {
            let field_expr = parse_expr(ctx)?;
            fields.push(field_expr);
            if !ctx.peek_is(TokenKind::Comma) {
                break;
            }
            ctx.expect(TokenKind::Comma)?;
        }
    }
    ctx.expect(TokenKind::RCurly)?;
    let struct_literal_expr = ExprKind::StructLiteral(struct_name, fields);
    Ok(ParsedExpr::new(ctx, offset, struct_literal_expr))
}

fn parse_call<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
    func_name: &'src str,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    ctx.expect(TokenKind::LParen)?;
    let mut args = Vec::new();
    if !ctx.peek_is(TokenKind::RParen) {
        loop {
            let arg_expr = parse_expr(ctx)?;
            args.push(arg_expr);
            if !ctx.peek_is(TokenKind::Comma) {
                break;
            }
            ctx.expect(TokenKind::Comma)?;
        }
    }
    ctx.expect(TokenKind::RParen)?;
    let call_expr = ExprKind::Call(func_name, args);
    Ok(ParsedExpr::new(ctx, offset, call_expr))
}

fn parse_if<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    let condition_expr = parse_expr(ctx)?;
    ctx.expect(TokenKind::Then)?;
    let then_expr = parse_expr(ctx)?;
    ctx.expect(TokenKind::Else)?;
    let else_expr = parse_expr(ctx)?;
    let if_expr = ExprKind::If(condition_expr, then_expr, else_expr);
    Ok(ParsedExpr::new(ctx, offset, if_expr))
}

fn parse_loop_expr<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
    constructor: impl Fn(
        Vec<&'ast ParsedLoopIterVar<'src, 'ast>>,
        &'ast ParsedExpr<'src, 'ast>,
    ) -> ExprKind<'src, 'ast, ()>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    ctx.expect(TokenKind::LSquare)?;
    let mut bindings = Vec::new();
    if !ctx.peek_is(TokenKind::RSquare) {
        loop {
            let Token { str: var_name, .. } = ctx.expect(TokenKind::Variable)?;
            ctx.expect(TokenKind::Colon)?;
            let expr = parse_expr(ctx)?;
            bindings.push(ParsedLoopIterVar::new(
                ctx,
                offset,
                LoopIterVar {
                    name: var_name,
                    expr,
                },
            ));
            if !ctx.peek_is(TokenKind::Comma) {
                break;
            }
            ctx.expect(TokenKind::Comma)?;
        }
    }
    ctx.expect(TokenKind::RSquare)?;
    let body = parse_expr(ctx)?;
    Ok(ParsedExpr::new(ctx, offset, constructor(bindings, body)))
}
