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

const OPERATORS: [(&'static str, TokenKind); 16] = [
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

pub fn lex<'a>(program: &'a str) -> Result<Vec<Token<'a>>, Vec<LexError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut curr_pos = 0;

    loop {
        let (result, next_pos) = next_token(program, curr_pos);
        curr_pos = next_pos;
        match result {
            Ok(token) => tokens.push(token),
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

fn next_token<'a>(program: &'a str, start: usize) -> (Result<Token<'a>, LexError>, usize) {
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
    if matches!(first_char, b'a'..=b'z' | b'A'..=b'Z') {
        return read_alpha(program, next);
    }

    // check punctuation + operators
    let hardcoded_tokens = PUNCTUATION.iter().chain(OPERATORS.iter());
    for (keyword, kind) in hardcoded_tokens {
        if program[next..].starts_with(keyword) {
            return (
                Ok(Token::new(
                    *kind,
                    next,
                    &program[next..next + keyword.len()],
                )),
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
        while next < program.len() && program.as_bytes()[next] == b'\n' {
            next += 1;
        }
        return (Ok(token), next);
    }

    // read numeric
    if matches!(first_char, b'0'..=b'9') {
        return read_numeric(program, next);
    }

    return (
        Err(LexError::IllegalCharacter(next, *first_char as char)),
        next + 1,
    );
}

fn skip_whitespace(program: &str, start: usize) -> usize {
    let mut pos = start;
    while pos < program.len() && program.as_bytes()[pos] == b' ' {
        pos += 1;
    }
    pos
}

fn skip_line_comment(program: &str, start: usize) -> usize {
    let mut pos = start;
    if program[start..].starts_with("//") {
        while pos < program.len() && program.as_bytes()[pos] != b'\n' {
            pos += 1;
        }
    }
    pos
}

fn skip_block_comment(program: &str, start: usize) -> usize {
    let mut pos = start;
    if program[start..].starts_with("/*") {
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
    if program[start..].starts_with("\\\n") {
        return start + 2;
    }
    start
}

fn read_alpha<'a>(program: &'a str, start: usize) -> (Result<Token<'a>, LexError>, usize) {
    let mut pos = start;
    while pos < program.len()
        && matches!(program.as_bytes()[pos], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
    {
        pos += 1;
    }
    let token_str = &program[start..pos];
    for keyword in KEYWORDS {
        if token_str == keyword.0 {
            return (Ok(Token::new(keyword.1, start, token_str)), pos);
        }
    }

    (Ok(Token::new(TokenKind::Variable, start, token_str)), pos)
}

fn read_string_literal<'a>(program: &'a str, start: usize) -> (Result<Token<'a>, LexError>, usize) {
    let mut pos = start + 1; // skip opening quote
    while pos < program.len() && program.as_bytes()[pos] != b'"' && program.as_bytes()[pos] != b'\n'
    {
        pos += 1;
    }

    if pos >= program.len() || program.as_bytes()[pos] != b'"' {
        return (Err(LexError::UnterminatedString(start)), pos);
    }
    return (
        Ok(Token::new(TokenKind::String, start, &program[start..=pos])),
        pos + 1,
    );
}

fn read_numeric<'a>(program: &'a str, start: usize) -> (Result<Token<'a>, LexError>, usize) {
    let mut pos = start;
    let mut has_decimal = false;

    while pos < program.len() {
        match program.as_bytes()[pos] {
            b'0'..=b'9' => pos += 1,
            b'.' if !has_decimal => {
                has_decimal = true;
                pos += 1;
            }
            _ => break,
        }
    }

    let token_str = &program[start..pos];
    if has_decimal {
        (Ok(Token::new(TokenKind::FloatVal, start, token_str)), pos)
    } else {
        (Ok(Token::new(TokenKind::IntVal, start, token_str)), pos)
    }
}
