use bumpalo::Bump;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::ast::{BinOp, ParseError, ParseErrorKind, UnOp},
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
            Some(token) => Err(ParseError::new(
                token.offset,
                ParseErrorKind::UnexpectedToken {
                    expected: vec![expected],
                    found: token.kind,
                    value: token.str,
                },
            )),
            None => Err(ParseError::new(
                self.tokens.last().map_or(0, |t| t.offset),
                ParseErrorKind::UnexpectedToken {
                    expected: vec![expected],
                    found: TokenKind::EndOfFile,
                    value: "",
                },
            )),
        }
    }

    pub(super) fn peek_is(&self, expected: TokenKind) -> bool {
        matches!(self.peek(), Some(token) if token.kind == expected)
    }

    pub(super) fn try_peek_unop(&self) -> Option<UnOp> {
        match self.peek() {
            Some(token) if token.kind == TokenKind::Op => match token.str {
                "-" => Some(UnOp::Neg),
                "!" => Some(UnOp::Not),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn try_peek_binop(&self) -> Option<BinOp> {
        match self.peek() {
            Some(token) if token.kind == TokenKind::Op => match token.str {
                "+" => Some(BinOp::Add),
                "-" => Some(BinOp::Sub),
                "*" => Some(BinOp::Mul),
                "/" => Some(BinOp::Div),
                "%" => Some(BinOp::Mod),
                "&&" => Some(BinOp::And),
                "||" => Some(BinOp::Or),
                "==" => Some(BinOp::Eq),
                "!=" => Some(BinOp::NotEq),
                "<" => Some(BinOp::Lt),
                "<=" => Some(BinOp::Lte),
                ">" => Some(BinOp::Gt),
                ">=" => Some(BinOp::Gte),
                _ => None,
            },
            _ => None,
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
        Err(ParseError::new(
            self.peek().map_or(0, |t| t.offset),
            ParseErrorKind::UnexpectedToken {
                expected: expected.to_vec(),
                found: self.peek().map_or(TokenKind::EndOfFile, |t| t.kind),
                value: self.peek().map_or("", |t| t.str),
            },
        ))
    }

    pub(super) fn alloc<A>(&self, value: A) -> &'ast A {
        self.alloc.alloc(value)
    }
}
