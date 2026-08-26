#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub offset: usize,
    pub str: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // primitive values
    True,
    False,
    FloatVal,
    IntVal,
    String,
    Variable,

    // keywords
    Array,
    Assert,
    Else,
    Fn,
    If,
    Image,
    Let,
    Print,
    Read,
    Show,
    Return,
    Struct,
    Sum,
    Then,
    Time,
    To,
    Void,
    Write,

    // type keywords
    BoolType,
    FloatType,
    IntType,

    // punctuation
    Colon,
    Comma,
    LCurly,
    LParen,
    LSquare,
    RCurly,
    RParen,
    RSquare,

    // operators
    Dot,
    Equals,
    Op,

    // misc
    EndOfFile,
    NewLine,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, offset: usize, str: &'a str) -> Self {
        Token { kind, offset, str }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexError {
    UnterminatedString(usize),
    IllegalCharacter(usize, char),
}
