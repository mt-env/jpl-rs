use bumpalo::Bump;

use crate::lexer::token::{Token, TokenKind};

struct ParserCtx<'src, 'ast> {
    alloc: &'ast mut Bump,
    tokens: Vec<Token<'src>>,
    curr_pos: usize,
}

impl<'src, 'ast> ParserCtx<'src, 'ast> {
    fn new(alloc: &'ast mut Bump, tokens: Vec<Token<'src>>) -> ParserCtx<'src, 'ast> {
        ParserCtx {
            alloc,
            tokens,
            curr_pos: 0,
        }
    }

    fn peek(&self) -> Option<Token<'src>> {
        self.tokens.get(self.curr_pos).copied()
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token<'src>, ()> {
        match self.peek() {
            Some(token) if token.kind == expected => {
                self.curr_pos += 1;
                Ok(token)
            }
            _ => todo!(),
        }
    }

    fn expect_many(&mut self, expected: &[TokenKind]) -> Result<Token<'src>, ()> {
        for expected_kind in expected {
            if let Ok(token) = self.expect(*expected_kind) {
                return Ok(token);
            }
        }
        todo!()
    }
}
