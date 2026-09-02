use crate::lexer::token::{IllegalByteError, LexError, Token, TokenKind};

pub mod token;

const KEYWORDS: [(&str, TokenKind); 23] = [
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

const PUNCTUATION: [(&str, TokenKind); 8] = [
    (":", TokenKind::Colon),
    (",", TokenKind::Comma),
    ("{", TokenKind::LCurly),
    ("(", TokenKind::LParen),
    ("[", TokenKind::LSquare),
    ("}", TokenKind::RCurly),
    (")", TokenKind::RParen),
    ("]", TokenKind::RSquare),
];

const OPERATORS: [(&str, TokenKind); 16] = [
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
    ("%", TokenKind::Op),
    ("=", TokenKind::Equals),
];

pub fn validate_source(program: Vec<u8>) -> Result<String, Vec<IllegalByteError>> {
    let mut errors = Vec::new();
    for (offset, byte) in program.iter().enumerate() {
        if (*byte >= 32 && *byte <= 126) || *byte == 10 {
            continue;
        }
        errors.push(IllegalByteError {
            offset,
            byte: *byte,
        });
    }
    if errors.is_empty() {
        unsafe { Ok(String::from_utf8(program).unwrap_unchecked()) } // safe because we just checked all bytes
    } else {
        Err(errors)
    }
}

pub fn lex(program: &str) -> Result<Vec<Token<'_>>, Vec<LexError>> {
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

fn next_token(program: &str, start: usize) -> (Result<Token<'_>, LexError>, usize) {
    // nuke all whitespace and comments at the start of the token
    let mut next = start;
    loop {
        let previous = next;
        next = skip_whitespace(program, next);
        next = skip_line_comment(program, next);
        next = skip_block_comment(program, next);
        next = skip_escaped_newline(program, next);

        if next == previous {
            break;
        }
    }

    let first_char = program.as_bytes().get(next);

    // sentinel value for end
    let Some(first_char) = first_char else {
        return (Ok(Token::new(TokenKind::EndOfFile, next, "")), next);
    };

    // if first letter is alphabetic, read until neither alphanumeric nor underscore
    if first_char.is_ascii_alphabetic() {
        return read_alpha(program, next);
    }

    // read numeric
    if first_char.is_ascii_digit()
        || (first_char == &b'.'
            && program
                .as_bytes()
                .get(next + 1)
                .is_some_and(u8::is_ascii_digit))
    {
        return read_numeric(program, next);
    }

    // check punctuation + operators
    let hardcoded_tokens = PUNCTUATION.iter().chain(OPERATORS.iter());
    for (keyword, kind) in hardcoded_tokens {
        if let Some(slice) = program.get(next..)
            && slice.starts_with(keyword)
        {
            return (Ok(Token::new(*kind, next, keyword)), next + keyword.len());
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

    (Err(LexError::IllegalCharacter(next, *first_char)), next + 1)
}

fn skip_whitespace(program: &str, start: usize) -> usize {
    let mut pos = start;
    while program.as_bytes().get(pos) == Some(&b' ') {
        pos += 1;
    }
    pos
}

fn skip_line_comment(program: &str, start: usize) -> usize {
    let mut pos = start;
    if let Some(slice) = program.get(start..)
        && slice.starts_with("//")
    {
        while let Some(c) = program.as_bytes().get(pos)
            && c != &b'\n'
        {
            pos += 1;
        }
    }
    pos
}

fn skip_block_comment(program: &str, start: usize) -> usize {
    let mut pos = start;
    if let Some(slice) = program.get(start..)
        && slice.starts_with("/*")
    {
        pos += 2;
        while pos < program.len() && !program[pos..].starts_with("*/") {
            pos += 1;
        }
        if pos < program.len() {
            pos += 2;
        }
    }
    pos
}

fn skip_escaped_newline(program: &str, start: usize) -> usize {
    if let Some(slice) = program.get(start..)
        && slice.starts_with("\\\n")
    {
        return start + 2;
    }
    start
}

fn read_alpha(program: &str, start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start;
    while let Some(&c) = program.as_bytes().get(pos)
        && (c.is_ascii_alphanumeric() || c == b'_')
    {
        pos += 1;
    }
    // safe - only way pos is OOB is if it's right at the end of the program
    // in which case indexing to the end of the program is valid
    let token_str = unsafe { program.get_unchecked(start..pos) };
    for keyword in KEYWORDS {
        if token_str == keyword.0 {
            return (Ok(Token::new(keyword.1, start, token_str)), pos);
        }
    }

    (Ok(Token::new(TokenKind::Variable, start, token_str)), pos)
}

fn read_string_literal(program: &str, start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start + 1; // skip opening quote

    while let Some(&c) = program.as_bytes().get(pos) {
        if c == b'"' {
            // safe - the `while let` guarantees we're in bounds
            let token_str = unsafe { program.get_unchecked(start..=pos) };
            return (Ok(Token::new(TokenKind::String, start, token_str)), pos + 1);
        }
        if c == b'\n' {
            return (Err(LexError::UnterminatedString(start)), pos);
        }
        pos += 1;
    }
    (Err(LexError::UnterminatedString(start)), pos)
}

fn read_numeric(program: &str, start: usize) -> (Result<Token<'_>, LexError>, usize) {
    let mut pos = start;
    let mut has_decimal = false;

    loop {
        match program.as_bytes().get(pos) {
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
        (Ok(Token::new(TokenKind::FloatVal, start, token_str)), pos)
    } else {
        (Ok(Token::new(TokenKind::IntVal, start, token_str)), pos)
    }
}
