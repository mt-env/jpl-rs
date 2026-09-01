use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ast::{Cmd, ParsedCmd},
        parse_expr, parse_lvalue,
        parser_ctx::ParserCtx,
    },
};

pub(super) fn parse_cmd<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let curr_token = ctx.expect_many(&[
        TokenKind::Read,
        TokenKind::Write,
        TokenKind::Let,
        TokenKind::Assert,
        TokenKind::Print,
        TokenKind::Show,
        TokenKind::Time,
    ])?;

    match curr_token.kind {
        TokenKind::Read => parse_read(ctx),
        TokenKind::Write => parse_write(ctx),
        TokenKind::Let => parse_let(ctx),
        TokenKind::Assert => parse_assert(ctx),
        TokenKind::Print => parse_print(ctx),
        TokenKind::Show => parse_show(ctx),
        TokenKind::Time => parse_time(ctx),
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    }
}

fn parse_read<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Read)?;
    ctx.expect(TokenKind::Image)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::To)?;
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Read(str, lvalue)))
}

fn parse_write<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Write)?;
    ctx.expect(TokenKind::Image)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::To)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Write(expr, str)))
}

fn parse_let<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Let)?;
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    ctx.expect(TokenKind::Equals)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Let(lvalue, expr)))
}

fn parse_assert<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Assert)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::Comma)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Assert(expr, str)))
}

fn parse_print<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Print)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Print(str)))
}

fn parse_show<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Show)?;
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Show(expr)))
}

fn parse_time<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    let Token { offset, .. } = ctx.expect(TokenKind::Time)?;
    let cmd = parse_cmd(ctx)?;
    ctx.expect(TokenKind::NewLine)?;
    Ok(ParsedCmd::new(ctx, offset, Cmd::Time(cmd)))
}
