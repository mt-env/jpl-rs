use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::TokenKind,
    parser::{ast::ParsedCmd, parser_ctx::ParserCtx},
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
    todo!()
}

fn parse_write<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}

fn parse_let<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}

fn parse_assert<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}

fn parse_print<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}

fn parse_show<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}

fn parse_time<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedCmd<'src, 'ast>, ()> {
    todo!()
}
