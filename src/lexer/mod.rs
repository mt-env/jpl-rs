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

        todo!("delegate to helpers based on first char")
    }
}
