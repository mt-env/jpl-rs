use std::hint::unreachable_unchecked;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{ParseError, ParsedType, Type},
    },
};

pub(super) fn parse_type<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedType<'src, 'ast>, ParseError<'src>> {
    // parse base type
    let Token { offset, kind, str } = ctx.expect_many(&[
        TokenKind::IntType,
        TokenKind::FloatType,
        TokenKind::BoolType,
        TokenKind::Void,
        TokenKind::Variable,
    ])?;
    let ty_kind = match kind {
        TokenKind::IntType => Type::Int,
        TokenKind::FloatType => Type::Float,
        TokenKind::BoolType => Type::Bool,
        TokenKind::Void => Type::Void,
        TokenKind::Variable => Type::Struct { name: str },
        _ => unsafe { unreachable_unchecked() }, // safe because of expect_many
    };

    let mut ty = ParsedType::make_parsed(ctx, offset, ty_kind);
    loop {
        if ctx.peek_is(TokenKind::LSquare) {
            ty = parse_array_type(ctx, offset, ty)?;
            continue;
        }
        break;
    }
    Ok(ty)
}

fn parse_array_type<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    offset: usize,
    element_type: &'ast ParsedType<'src, 'ast>,
) -> Result<&'ast ParsedType<'src, 'ast>, ParseError<'src>> {
    ctx.expect(TokenKind::LSquare)?;
    let mut dimension = 1;
    while !ctx.peek_is(TokenKind::RSquare) {
        ctx.expect(TokenKind::Comma)?;
        dimension += 1;
    }
    ctx.expect(TokenKind::RSquare)?;
    Ok(ParsedType::make_parsed(
        ctx,
        offset,
        Type::Array {
            element_type,
            dimension,
        },
    ))
}
