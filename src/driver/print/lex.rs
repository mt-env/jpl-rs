use std::fmt::{Display, Formatter};

use crate::lexer::token::{Token, TokenKind};

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{token}");
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
            Self::True => write!(f, "TRUE"),
            Self::False => write!(f, "FALSE"),
            Self::FloatVal => write!(f, "FLOATVAL"),
            Self::IntVal => write!(f, "INTVAL"),
            Self::String => write!(f, "STRING"),
            Self::Variable => write!(f, "VARIABLE"),
            Self::Array => write!(f, "ARRAY"),
            Self::Assert => write!(f, "ASSERT"),
            Self::Else => write!(f, "ELSE"),
            Self::Fn => write!(f, "FN"),
            Self::If => write!(f, "IF"),
            Self::Image => write!(f, "IMAGE"),
            Self::Let => write!(f, "LET"),
            Self::Print => write!(f, "PRINT"),
            Self::Read => write!(f, "READ"),
            Self::Show => write!(f, "SHOW"),
            Self::Return => write!(f, "RETURN"),
            Self::Struct => write!(f, "STRUCT"),
            Self::Sum => write!(f, "SUM"),
            Self::Then => write!(f, "THEN"),
            Self::Time => write!(f, "TIME"),
            Self::To => write!(f, "TO"),
            Self::Void => write!(f, "VOID"),
            Self::Write => write!(f, "WRITE"),
            Self::BoolType => write!(f, "BOOL"),
            Self::FloatType => write!(f, "FLOAT"),
            Self::IntType => write!(f, "INT"),
            Self::Colon => write!(f, "COLON"),
            Self::Comma => write!(f, "COMMA"),
            Self::LCurly => write!(f, "LCURLY"),
            Self::LParen => write!(f, "LPAREN"),
            Self::LSquare => write!(f, "LSQUARE"),
            Self::RCurly => write!(f, "RCURLY"),
            Self::RParen => write!(f, "RPAREN"),
            Self::RSquare => write!(f, "RSQUARE"),
            Self::Dot => write!(f, "DOT"),
            Self::Equals => write!(f, "EQUALS"),
            Self::Op => write!(f, "OP"),
            Self::EndOfFile => write!(f, "END_OF_FILE"),
            Self::NewLine => write!(f, "NEWLINE"),
        }
    }
}
