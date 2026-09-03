use bumpalo::Bump;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::ast::ParseError,
};

pub(super) struct ParserCtx<'src, 'ast> {
    alloc: &'ast Bump,
    tokens: Vec<Token<'src>>,
    curr_pos: usize,
}

impl<'src, 'ast> ParserCtx<'src, 'ast> {
    pub(super) const fn new(alloc: &'ast Bump, tokens: Vec<Token<'src>>) -> Self {
        ParserCtx {
            alloc,
            tokens,
            curr_pos: 0,
        }
    }

    pub(super) fn peek(&self) -> Option<Token<'src>> {
        self.tokens.get(self.curr_pos).copied()
    }

    pub(super) fn expect(&mut self, expected: TokenKind) -> Result<Token<'src>, ParseError<'src>> {
        match self.peek() {
            Some(token) if token.kind == expected => {
                self.curr_pos += 1;
                Ok(token)
            }
            _ => todo!(),
        }
    }

    pub(super) fn peek_is(&self, expected: TokenKind) -> bool {
        match self.peek() {
            Some(token) if token.kind == expected => true,
            _ => todo!(),
        }
    }

    pub(super) fn expect_many(
        &mut self,
        expected: &[TokenKind],
    ) -> Result<Token<'src>, ParseError<'src>> {
        for expected_kind in expected {
            if let Ok(token) = self.expect(*expected_kind) {
                return Ok(token);
            }
        }
        todo!()
    }

    pub(super) fn alloc<A>(&self, value: A) -> &'ast A {
        self.alloc.alloc(value)
    }
}
