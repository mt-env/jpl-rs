use crate::lexer::token::{LexError, Token, TokenKind};

pub mod token;

const KEYWORDS: [(&'static str, TokenKind); 23] = [
    ("array", TokenKind::Array),
    ("assert", TokenKind::Assert),
    ("bool", TokenKind::BoolType),
    ("else", TokenKind::Else),
    ("false", TokenKind::False),
    ("float", TokenKind::FloatType),
    ("fn", TokenKind::Fn),
    ("if", TokenKind::If),
    ("image", TokenKind::Image),
    ("int", TokenKind::IntType),
    ("let", TokenKind::Let),
    ("print", TokenKind::Print),
    ("read", TokenKind::Read),
    ("return", TokenKind::Return),
    ("show", TokenKind::Show),
    ("struct", TokenKind::Struct),
    ("sum", TokenKind::Sum),
    ("then", TokenKind::Then),
    ("time", TokenKind::Time),
    ("to", TokenKind::To),
    ("true", TokenKind::True),
    ("void", TokenKind::Void),
    ("write", TokenKind::Write),
];

const PUNCTUATION: [(&'static str, TokenKind); 8] = [
    (":", TokenKind::Colon),
    (",", TokenKind::Comma),
    ("{", TokenKind::LCurly),
    ("(", TokenKind::LParen),
    ("[", TokenKind::LSquare),
    ("}", TokenKind::RCurly),
    (")", TokenKind::RParen),
    ("]", TokenKind::RSquare),
];

const OPERATORS: [(&'static str, TokenKind); 15] = [
    ("&&", TokenKind::Op),
    ("||", TokenKind::Op),
    ("==", TokenKind::Op),
    ("!=", TokenKind::Op),
    ("<=", TokenKind::Op),
    (">=", TokenKind::Op),
    ("+", TokenKind::Op),
    ("-", TokenKind::Op),
    ("*", TokenKind::Op),
    ("/", TokenKind::Op),
    ("<", TokenKind::Op),
    (">", TokenKind::Op),
    ("!", TokenKind::Op),
    (".", TokenKind::Dot),
    ("=", TokenKind::Equals),
];

