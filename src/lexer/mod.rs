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
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors = Vec::new();
    let mut curr_pos = 0;

    loop {
        let (result, next_pos) = next_token(program, curr_pos);
        curr_pos = next_pos;
        match result {
            Ok(token) => {
                // skip consecutive newlines
                if token.kind == TokenKind::NewLine
                    && let Some(last_token) = tokens.last()
                    && last_token.kind == TokenKind::NewLine
                {
                    continue;
                }
                tokens.push(token);
            }
            Err(error) => errors.push(error),
        }

        if let Some(token) = tokens.last()
            && token.kind == TokenKind::EndOfFile
        {
            break;
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

fn next_token(program: &[u8], start: usize) -> (Result<Token<'_>, LexError>, usize) {
    // nuke all whitespace and comments at the start of the token
    let mut next = start;
    loop {
        let previous = next;
        next = skip_whitespace(program, next);
        next = skip_line_comment(program, next);
        next = match skip_block_comment(program, next) {
            (pos, Ok(())) => pos,
            (pos, Err(e)) => return (Err(e), pos),
        };
        next = skip_escaped_newline(program, next);

        if next == previous {
            break;
        }
    }

    let first_char = program.get(next);

    // sentinel value for end
    let Some(first_char) = first_char else {
        return (Ok(Token::new(TokenKind::EndOfFile, next, "")), next);
    };

    // check for invalid bytes
    if (*first_char < 32 || *first_char > 126) && *first_char != 10 {
        return (
            Err(LexError::new(next, LexErrorKind::IllegalByte(*first_char))),
            next + 1,
        );
    }

    // if first letter is alphabetic, read until neither alphanumeric nor underscore
    if first_char.is_ascii_alphabetic() {
        return read_alpha(program, next);
    }

    // read numeric
    if first_char.is_ascii_digit()
        || (first_char == &b'.' && program.get(next + 1).is_some_and(u8::is_ascii_digit))
    {
        return read_numeric(program, next);
    }

    // check punctuation + operators
    let hardcoded_tokens = PUNCTUATION.iter().chain(OPERATORS.iter());
    for (keyword, kind) in hardcoded_tokens {
        if let Some(slice) = program.get(next..)
            && slice.starts_with(keyword)
        {
            return (
                Ok(Token::from_u8(*kind, next, keyword)),
                next + keyword.len(),
            );
        }
    }

    // read string literal
    if first_char == &b'"' {
        return read_string_literal(program, next);
    }

    // read newline
    if first_char == &b'\n' {
        let token = Token::new(TokenKind::NewLine, next, "\n");
        return (Ok(token), next + 1);
    }

    (
        Err(LexError::new(
            next,
            LexErrorKind::IllegalCharacter(*first_char),
        )),
        next + 1,
    )
}

fn skip_whitespace(program: &[u8], start: usize) -> usize {
    let mut pos = start;
    while program.get(pos) == Some(&b' ') {
        pos += 1;
    }
    pos
}

fn skip_line_comment(program: &[u8], start: usize) -> usize {
    let mut pos = start;
    if let Some(slice) = program.get(start..)
        && slice.starts_with(b"//")
    {
        while let Some(c) = program.get(pos)
            && c != &b'\n'
        {
            pos += 1;
        }
    }
    pos
}

fn skip_block_comment(program: &[u8], start: usize) -> (usize, Result<(), LexError>) {
    let mut pos = start;
    if let Some(slice) = program.get(start..)
        && slice.starts_with(b"/*")
    {
        pos += 2;

        while let Some(slice) = program.get(pos..)
            && !slice.starts_with(b"*/")
        {
            pos += 1;
        }

        if program.get(pos..).is_none() {
            return (
                pos,
                Err(LexError::new(start, LexErrorKind::UnterminatedComment)),
            );
        }

        pos += 2;
    }
    (pos, Ok(()))
}

fn skip_escaped_newline(program: &[u8], start: usize) -> usize {
    if let Some(slice) = program.get(start..)
        && slice.starts_with(b"\\\n")
    {
        return start + 2;
    }
    start
}

fn read_alpha(program: &[u8], start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start;
    while let Some(&c) = program.get(pos)
        && (c.is_ascii_alphanumeric() || c == b'_')
    {
        pos += 1;
    }
    // safe - only way pos is OOB is if it's right at the end of the program
    // in which case indexing to the end of the program is valid
    let token_str = unsafe { program.get_unchecked(start..pos) };
    if let Some(&token_kind) = KEYWORDS.get(token_str) {
        return (Ok(Token::from_u8(token_kind, start, token_str)), pos);
    }

    (
        Ok(Token::from_u8(TokenKind::Variable, start, token_str)),
        pos,
    )
}

fn read_string_literal(program: &[u8], start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start + 1; // skip opening quote

    while let Some(&c) = program.get(pos) {
        if c == b'"' {
            // safe - the `while let` guarantees we're in bounds
            let token_str = unsafe { program.get_unchecked(start..=pos) };
            return (
                Ok(Token::from_u8(TokenKind::String, start, token_str)),
                pos + 1,
            );
        }
        if c == b'\n' {
            return (
                Err(LexError::new(start, LexErrorKind::UnterminatedString)),
                pos,
            );
        }
        pos += 1;
    }
    (
        Err(LexError::new(start, LexErrorKind::UnterminatedString)),
        pos,
    )
}

fn read_numeric(program: &[u8], start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start;
    let mut has_decimal = false;

    loop {
        match program.get(pos) {
            Some(b'0'..=b'9') => pos += 1,
            Some(b'.') if !has_decimal => {
                has_decimal = true;
                pos += 1;
            }
            _ => break,
        }
    }

    // safe - only way pos is OOB is if it's right at the end of the program
    // in which case indexing to the end of the program is valid
    let token_str = unsafe { program.get_unchecked(start..pos) };
    if has_decimal {
        (
            Ok(Token::from_u8(TokenKind::FloatVal, start, token_str)),
            pos,
        )
    } else {
        (Ok(Token::from_u8(TokenKind::IntVal, start, token_str)), pos)
    }
}
