use std::fmt::{Display, Formatter};

use crate::lexer::token::{Token, TokenKind};

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{}", token);
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.kind == TokenKind::NewLine || self.kind == TokenKind::EndOfFile {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "{} '{}'", self.kind, self.str)
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::True => write!(f, "TRUE"),
            TokenKind::False => write!(f, "FALSE"),
            TokenKind::FloatVal => write!(f, "FLOATVAL"),
            TokenKind::IntVal => write!(f, "INTVAL"),
            TokenKind::String => write!(f, "STRING"),
            TokenKind::Variable => write!(f, "VARIABLE"),
            TokenKind::Array => write!(f, "ARRAY"),
            TokenKind::Assert => write!(f, "ASSERT"),
            TokenKind::Else => write!(f, "ELSE"),
            TokenKind::Fn => write!(f, "FN"),
            TokenKind::If => write!(f, "IF"),
            TokenKind::Image => write!(f, "IMAGE"),
            TokenKind::Let => write!(f, "LET"),
            TokenKind::Print => write!(f, "PRINT"),
            TokenKind::Read => write!(f, "READ"),
            TokenKind::Show => write!(f, "SHOW"),
            TokenKind::Return => write!(f, "RETURN"),
            TokenKind::Struct => write!(f, "STRUCT"),
            TokenKind::Sum => write!(f, "SUM"),
            TokenKind::Then => write!(f, "THEN"),
            TokenKind::Time => write!(f, "TIME"),
            TokenKind::To => write!(f, "TO"),
            TokenKind::Void => write!(f, "VOID"),
            TokenKind::Write => write!(f, "WRITE"),
            TokenKind::BoolType => write!(f, "BOOL"),
            TokenKind::FloatType => write!(f, "FLOAT"),
            TokenKind::IntType => write!(f, "INT"),
            TokenKind::Colon => write!(f, "COLON"),
            TokenKind::Comma => write!(f, "COMMA"),
            TokenKind::LCurly => write!(f, "LCURLY"),
            TokenKind::LParen => write!(f, "LPAREN"),
            TokenKind::LSquare => write!(f, "LSQUARE"),
            TokenKind::RCurly => write!(f, "RCURLY"),
            TokenKind::RParen => write!(f, "RPAREN"),
            TokenKind::RSquare => write!(f, "RSQUARE"),
            TokenKind::Dot => write!(f, "DOT"),
            TokenKind::Equals => write!(f, "EQUALS"),
            TokenKind::Op => write!(f, "OP"),
            TokenKind::EndOfFile => write!(f, "END_OF_FILE"),
            TokenKind::NewLine => write!(f, "NEWLINE"),
        }
    }
}
