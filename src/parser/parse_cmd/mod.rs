use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ast::{Cmd, ParseError, ParsedCmd},
        parse_binding, parse_expr, parse_lvalue, parse_stmt, parse_type,
        parser_ctx::ParserCtx,
    },
};

pub(super) fn parse_cmd<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ParseError<'src>> {
    let Token {
        kind,
        offset,
        str: _,
    } = ctx.expect_many(&[
        TokenKind::Read,
        TokenKind::Write,
        TokenKind::Let,
        TokenKind::Assert,
        TokenKind::Print,
        TokenKind::Show,
        TokenKind::Time,
        TokenKind::Fn,
        TokenKind::Struct,
    ])?;

    let cmd_kind = match kind {
        TokenKind::Read => parse_read(ctx),
        TokenKind::Write => parse_write(ctx),
        TokenKind::Let => parse_let(ctx),
        TokenKind::Assert => parse_assert(ctx),
        TokenKind::Print => parse_print(ctx),
        TokenKind::Show => parse_show(ctx),
        TokenKind::Time => parse_time(ctx),
        TokenKind::Fn => parse_fn(ctx),
        TokenKind::Struct => parse_struct(ctx),
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };
    Ok(ParsedCmd::new(ctx, offset, cmd_kind?))
}

fn parse_read<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    ctx.expect(TokenKind::Image)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::To)?;
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Read(str, lvalue))
}

fn parse_write<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    ctx.expect(TokenKind::Image)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::To)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Write(expr, str))
}

fn parse_let<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    ctx.expect(TokenKind::Equals)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Let(lvalue, expr))
}

fn parse_assert<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::Comma)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Assert(expr, str))
}

fn parse_print<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Print(str))
}

fn parse_show<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Show(expr))
}

fn parse_time<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    let cmd = parse_cmd(ctx)?;
    Ok(Cmd::Time(cmd))
}

fn parse_fn<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    // function header
    let Token { str: name, .. } = ctx.expect(TokenKind::Variable)?;
    ctx.expect(TokenKind::LParen)?;
    let mut params = Vec::new();
    if !ctx.peek_is(TokenKind::RParen) {
        loop {
            params.push(parse_binding::parse_binding(ctx)?);
            if !ctx.peek_is(TokenKind::Comma) {
                break;
            }
            ctx.expect(TokenKind::Comma)?;
        }
    }
    ctx.expect(TokenKind::RParen)?;
    ctx.expect(TokenKind::Colon)?;
    let return_type = parse_type::parse_type(ctx)?;
    ctx.expect(TokenKind::LCurly)?;
    ctx.expect(TokenKind::NewLine)?;

    // statements in the function body
    let mut body = Vec::new();
    while !ctx.peek_is(TokenKind::RCurly) {
        body.push(parse_stmt::parse_stmt(ctx)?);
        ctx.expect(TokenKind::NewLine)?;
    }
    ctx.expect(TokenKind::RCurly)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Fn {
        name,
        params,
        return_type,
        body,
    })
}

fn parse_struct<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Cmd<'src, 'ast, ()>, ParseError<'src>> {
    // struct header
    let Token { str: name, .. } = ctx.expect(TokenKind::Variable)?;
    ctx.expect(TokenKind::LCurly)?;
    ctx.expect(TokenKind::NewLine)?;

    // struct fields
    let mut fields = Vec::new();
    while !ctx.peek_is(TokenKind::RCurly) {
        let Token {
            str: field_name, ..
        } = ctx.expect(TokenKind::Variable)?;
        ctx.expect(TokenKind::Colon)?;
        let field_type = parse_type::parse_type(ctx)?;
        ctx.expect(TokenKind::NewLine)?;
        fields.push((field_name, field_type));
    }

    ctx.expect(TokenKind::RCurly)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(Cmd::Struct { name, fields })
}
