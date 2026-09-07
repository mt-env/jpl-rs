use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{ParseError, ParsedStmt, Stmt},
        parse_expr, parse_lvalue,
    },
};

pub(super) fn parse_stmt<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedStmt<'src, 'ast>, ParseError<'src>> {
    let Token {
        kind,
        offset,
        str: _,
    } = ctx.expect_many(&[TokenKind::Let, TokenKind::Assert, TokenKind::Return])?;

    let stmt_kind = match kind {
        TokenKind::Let => parse_let(ctx)?,
        TokenKind::Assert => parse_assert(ctx)?,
        TokenKind::Return => parse_return(ctx)?,
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };
    Ok(ParsedStmt::new(ctx, offset, stmt_kind))
}

fn parse_let<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Stmt<'src, 'ast, ()>, ParseError<'src>> {
    let lvalue = parse_lvalue::parse_lvalue(ctx)?;
    ctx.expect(TokenKind::Equals)?;
    let expr = parse_expr::parse_expr(ctx)?;
    Ok(super::ast::Stmt::Let(lvalue, expr))
}

fn parse_assert<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Stmt<'src, 'ast, ()>, ParseError<'src>> {
    let expr = parse_expr::parse_expr(ctx)?;
    ctx.expect(TokenKind::Comma)?;
    let Token { str, .. } = ctx.expect(TokenKind::String)?;
    Ok(Stmt::Assert(expr, str))
}

fn parse_return<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<Stmt<'src, 'ast, ()>, ParseError<'src>> {
    let expr = parse_expr::parse_expr(ctx)?;
    Ok(Stmt::Return(expr))
}
