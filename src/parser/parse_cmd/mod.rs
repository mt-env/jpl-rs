use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ast::{Cmd, ParseError, ParsedCmd},
        parse_expr, parse_lvalue,
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
    ])?;

    let cmd_kind = match kind {
        TokenKind::Read => parse_read(ctx),
        TokenKind::Write => parse_write(ctx),
        TokenKind::Let => parse_let(ctx),
        TokenKind::Assert => parse_assert(ctx),
        TokenKind::Print => parse_print(ctx),
        TokenKind::Show => parse_show(ctx),
        TokenKind::Time => parse_time(ctx),
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
