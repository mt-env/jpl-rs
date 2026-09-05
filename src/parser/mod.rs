use bumpalo::Bump;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ast::{ParseError, ParsedProgram},
        parser_ctx::ParserCtx,
    },
};

pub mod ast;
mod parse_cmd;
mod parse_expr;
mod parse_lvalue;
mod parser_ctx;

pub fn parse<'src, 'ast>(
    alloc: &'ast mut Bump,
    tokens: Vec<Token<'src>>,
) -> Result<ParsedProgram<'src, 'ast>, ParseError<'src>> {
    let mut parser_ctx = ParserCtx::new(alloc, tokens);
    let mut parsed_program = Vec::new();
    while let Some(token) = parser_ctx.peek()
        && token.kind != TokenKind::EndOfFile
    {
        parsed_program.push(parse_cmd::parse_cmd(&mut parser_ctx)?);
    }

    Ok(parsed_program)
}
