use core::str;

use crate::lexer::token::{LexError, LexErrorKind, Token, TokenKind};

pub mod token;

static KEYWORDS: phf::Map<&[u8], TokenKind> = phf::phf_map! {
    b"array" => TokenKind::Array,
    b"assert" => TokenKind::Assert,
    b"bool" => TokenKind::BoolType,
    b"else" => TokenKind::Else,
    b"false" => TokenKind::False,
    b"float" => TokenKind::FloatType,
    b"fn" => TokenKind::Fn,
    b"if" => TokenKind::If,
    b"image" => TokenKind::Image,
    b"int" => TokenKind::IntType,
    b"let" => TokenKind::Let,
    b"print" => TokenKind::Print,
    b"read" => TokenKind::Read,
    b"return" => TokenKind::Return,
    b"show" => TokenKind::Show,
    b"struct" => TokenKind::Struct,
    b"sum" => TokenKind::Sum,
    b"then" => TokenKind::Then,
    b"time" => TokenKind::Time,
    b"to" => TokenKind::To,
    b"true" => TokenKind::True,
    b"void" => TokenKind::Void,
    b"write" => TokenKind::Write
};

const PUNCTUATION: [(&[u8], TokenKind); 8] = [
    (b":", TokenKind::Colon),
    (b",", TokenKind::Comma),
    (b"{", TokenKind::LCurly),
    (b"(", TokenKind::LParen),
    (b"[", TokenKind::LSquare),
    (b"}", TokenKind::RCurly),
    (b")", TokenKind::RParen),
    (b"]", TokenKind::RSquare),
];

const OPERATORS: [(&[u8], TokenKind); 16] = [
    (b"&&", TokenKind::Op),
    (b"||", TokenKind::Op),
    (b"==", TokenKind::Op),
    (b"!=", TokenKind::Op),
    (b"<=", TokenKind::Op),
    (b">=", TokenKind::Op),
    (b"+", TokenKind::Op),
    (b"-", TokenKind::Op),
    (b"*", TokenKind::Op),
    (b"/", TokenKind::Op),
    (b"<", TokenKind::Op),
    (b">", TokenKind::Op),
    (b"!", TokenKind::Op),
    (b".", TokenKind::Dot),
    (b"%", TokenKind::Op),
    (b"=", TokenKind::Equals),
];

pub fn lex(program: &[u8]) -> Result<Vec<Token<'_>>, Vec<LexError>> {
    Lexer::new(program).lex()
}

struct Lexer<'src> {
    program: &'src [u8],
    curr_pos: usize,
    tokens: Vec<Token<'src>>,
    errors: Vec<LexError>,
}

impl<'src> Lexer<'src> {
    fn new(program: &'src [u8]) -> Self {
        Self {
            program,
            curr_pos: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn at_end(&self) -> bool {
        self.curr_pos >= self.program.len()
    }

    fn lex(mut self) -> Result<Vec<Token<'src>>, Vec<LexError>> {
        // loop - maximal munch based on peeked current char until end
        while !self.at_end() {
            self.next();
        }

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    fn next(&mut self) {
        // eof sentinel value
        let Some(curr_char) = self.program.get(self.curr_pos) else {
            self.tokens
                .push(Token::new(TokenKind::EndOfFile, self.curr_pos, ""));
            return;
        };

        if curr_char.is_ascii_alphabetic() {
            self.lex_alpha();
            return;
        }

        if self.program.starts_with(b"//") {
            self.skip_line_comment();
            return;
        }

        todo!("delegate to helpers based on first char")
    }

    fn lex_alpha(&mut self) {
        let start = self.curr_pos;
        let end = self.program[start..]
            .iter()
            .position(|&c| !(c.is_ascii_alphanumeric() || c == b'_'))
            .map(|pos| start + pos)
            .unwrap_or(self.program.len());
        let token_str = &self.program[start..end];
        let tokenkind = KEYWORDS.get(token_str).unwrap_or(&TokenKind::Variable);
        self.curr_pos = end;
        self.tokens
            .push(Token::from_u8(*tokenkind, start, token_str));
    }

    fn skip_line_comment(&mut self) {
        let start = self.curr_pos;
        for (i, c) in self.program[start..].iter().enumerate() {
            if !valid_char(*c) {
                self.errors
                    .push(LexError::new(start + i, LexErrorKind::IllegalByte(*c)));
            }
            if *c == b'\n' {
                self.curr_pos = start + i + 1;
                return;
            }
        }
        self.curr_pos = self.program.len();
    }
}

fn valid_char(c: u8) -> bool {
    (c >= 32 && c <= 126) || c == 10
}
