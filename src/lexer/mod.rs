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

const TWO_CHAR_OPERATORS: phf::Set<&[u8]> = phf::phf_set! {
    b"&&",
    b"||",
    b"==",
    b"!=",
    b"<=",
    b">=",
};

const ONE_CHAR_OPERATORS: phf::Set<u8> = phf::phf_set! {
    b'+',
    b'-',
    b'*',
    b'/',
    b'<',
    b'>',
    b'!',
    b'%',
};

// one character punctuation tokens
const PUNCTUATION: phf::Map<u8, TokenKind> = phf::phf_map! {
    b':'=> TokenKind::Colon,
    b','=> TokenKind::Comma,
    b'.'=> TokenKind::Dot,
    b'=' => TokenKind::Equals,
    b'{'=> TokenKind::LCurly,
    b'('=> TokenKind::LParen,
    b'['=> TokenKind::LSquare,
    b'}'=> TokenKind::RCurly,
    b')'=> TokenKind::RParen,
    b']'=> TokenKind::RSquare,
    b'\n'=> TokenKind::NewLine,
};

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

        // dedup newlines and add EoF token
        self.tokens
            .dedup_by(|a, b| a.kind == TokenKind::NewLine && b.kind == TokenKind::NewLine);
        self.tokens
            .push(Token::new(TokenKind::EndOfFile, self.curr_pos, ""));

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    fn next(&mut self) {
        // return - add EoF in lex method
        let Some(curr_char) = self.program.get(self.curr_pos) else {
            return;
        };

        // check for illegal bytes first
        if !valid_char(*curr_char) {
            self.errors.push(LexError::new(
                self.curr_pos,
                LexErrorKind::IllegalByte(*curr_char),
            ));
            self.curr_pos += 1;
            return;
        }

        // eat whitespace
        if *curr_char == b' ' {
            self.curr_pos += 1;
            return;
        }

        // check for alphanumeric identifiers and keywords
        if curr_char.is_ascii_alphabetic() {
            self.lex_alpha();
            return;
        }

        // check for int/float literals. floats can start with a dot, yay -_-
        if curr_char.is_ascii_digit()
            || (*curr_char == b'.'
                && self
                    .program
                    .get(self.curr_pos + 1)
                    .is_some_and(u8::is_ascii_digit))
        {
            self.lex_numeric();
            return;
        }

        if *curr_char == b'"' {
            self.lex_string_literal();
            return;
        }

        // delegate based on the first two characters
        // comments and two character operators must be checked first
        if let Some(two_char) = self.program.get(self.curr_pos..self.curr_pos + 2) {
            // line comment
            if two_char == b"//" {
                self.skip_line_comment();
                return;
            }

            // block comment
            if two_char == b"/*" {
                self.skip_block_comment();
                return;
            }

            // escaped newline
            if two_char == b"\\\n" {
                self.curr_pos += 2;
                return;
            }

            // two character operators
            if TWO_CHAR_OPERATORS.contains(two_char) {
                let start = self.curr_pos;
                let end = self.curr_pos + 2;
                self.tokens.push(Token::from_u8(
                    TokenKind::Op,
                    start,
                    &self.program[start..end],
                ));
                self.curr_pos = end;
                return;
            }
        }

        // one char operators
        if ONE_CHAR_OPERATORS.contains(curr_char) {
            let start = self.curr_pos;
            let end = self.curr_pos + 1;
            self.tokens.push(Token::from_u8(
                TokenKind::Op,
                start,
                &self.program[start..end],
            ));
            self.curr_pos = end;
            return;
        }

        // punctuation - must be done after operators because some punctuation (e.g. '=') can be part of an operator
        // also must be done after numbers because numbers can start with a dot
        if let Some(tokenkind) = PUNCTUATION.get(curr_char) {
            let start = self.curr_pos;
            let end = self.curr_pos + 1;
            self.tokens
                .push(Token::from_u8(*tokenkind, start, &self.program[start..end]));
            self.curr_pos = end;
            return;
        }

        // if we reach here, we have an illegal character
        self.errors.push(LexError::new(
            self.curr_pos,
            LexErrorKind::IllegalByte(*curr_char),
        ));
        self.curr_pos += 1;
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
        self.tokens
            .push(Token::from_u8(*tokenkind, start, token_str));
        self.curr_pos = end;
    }

    fn lex_numeric(&mut self) {
        let start = self.curr_pos;
        let mut end = start;
        let mut has_dot = false;
        loop {
            match self.program.get(end) {
                Some(b'0'..=b'9') => end += 1,
                Some(b'.') if !has_dot => {
                    has_dot = true;
                    end += 1;
                }
                _ => break,
            }
        }
        let token_str = &self.program[start..end];
        let tokenkind = if has_dot {
            TokenKind::FloatVal
        } else {
            TokenKind::IntVal
        };
        self.tokens
            .push(Token::from_u8(tokenkind, start, token_str));
        self.curr_pos = end;
    }

    fn lex_string_literal(&mut self) {
        let start = self.curr_pos;
        let mut end = start + 1;
        loop {
            let Some(&c) = self.program.get(end) else {
                self.errors
                    .push(LexError::new(start, LexErrorKind::UnterminatedString));
                break;
            };
            if !valid_char(c) {
                self.errors
                    .push(LexError::new(end, LexErrorKind::IllegalByte(c)));
            }
            if c == b'"' {
                end += 1;
                break;
            }
            if c == b'\n' {
                self.errors
                    .push(LexError::new(start, LexErrorKind::UnterminatedString));
                break;
            }
            end += 1;
        }
        let token_str = &self.program[start..end];
        self.tokens
            .push(Token::from_u8(TokenKind::String, start, token_str));
        self.curr_pos = end;
    }

    fn skip_line_comment(&mut self) {
        let start = self.curr_pos;
        for (i, c) in self.program[start..].iter().enumerate() {
            if !valid_char(*c) {
                self.errors
                    .push(LexError::new(start + i, LexErrorKind::IllegalByte(*c)));
            }
            if *c == b'\n' {
                self.curr_pos = start + i;
                return;
            }
        }
        self.curr_pos = self.program.len();
    }

    fn skip_block_comment(&mut self) {
        let start = self.curr_pos;
        for (i, c) in self.program[start..].iter().enumerate() {
            if !valid_char(*c) {
                self.errors
                    .push(LexError::new(start + i, LexErrorKind::IllegalByte(*c)));
            }
            if *c == b'*' && self.program.get(start + i + 1) == Some(&b'/') {
                self.curr_pos = start + i + 2;
                return;
            }
        }
        self.errors
            .push(LexError::new(start, LexErrorKind::UnterminatedComment));
        self.curr_pos = self.program.len();
    }
}

fn valid_char(c: u8) -> bool {
    (c >= 32 && c <= 126) || c == 10
}
