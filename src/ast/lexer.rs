use std::fmt;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Num(String), 
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Eq,
    LParen,
    RParen,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize, // byte offset, inclusive
    pub end: usize,   // byte offset, exclusive
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    MalformedNumber(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub pos: usize, // byte offset where the problem starts
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexErrorKind::UnexpectedChar(c) => {
                write!(f, "unexpected character '{}' at position {}", c, self.pos)
            }
            LexErrorKind::MalformedNumber(s) => {
                write!(f, "malformed number '{}' at position {}", s, self.pos)
            }
        }
    }
}

impl std::error::Error for LexError {}

type Chars<'a> = Peekable<CharIndices<'a>>;

/// Turn source text into tokens. Never panics; bad input returns `Err`.
pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars: Chars = src.char_indices().peekable();

    while let Some(&(start, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() || c == '.' {
            let end = scan_number(src, &mut chars, start)?;
            tokens.push(Token {
                kind: TokenKind::Num(src[start..end].to_string()),
                start,
                end,
            });
        } else if c.is_alphabetic() || c == '_' {
            let mut end = start;
            while let Some(&(i, ch)) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    end = i + ch.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token {
                kind: TokenKind::Ident(src[start..end].to_string()),
                start,
                end,
            });
        } else {
            let kind = match c {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '^' => TokenKind::Caret,
                '=' => TokenKind::Eq,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                ',' => TokenKind::Comma,
                _ => {
                    return Err(LexError {
                        kind: LexErrorKind::UnexpectedChar(c),
                        pos: start,
                    })
                }
            };
            chars.next();
            tokens.push(Token { kind, start, end: start + c.len_utf8() });
        }
    }
    Ok(tokens)
}

/// Consumes a number starting at `start`; returns the end offset.
/// Accepts: 12, 0.25, .5   Rejects: 1.  .  1.2.3
fn scan_number(src: &str, chars: &mut Chars<'_>, start: usize) -> Result<usize, LexError> {
    let mut end = start;

    // integer part (may be empty, as in ".5")
    while let Some(&(i, c)) = chars.peek() {
        if c.is_ascii_digit() {
            end = i + 1;
            chars.next();
        } else {
            break;
        }
    }

    // fractional part
    if let Some(&(i, '.')) = chars.peek() {
        chars.next();
        end = i + 1;
        let frac_start = end;
        while let Some(&(j, c)) = chars.peek() {
            if c.is_ascii_digit() {
                end = j + 1;
                chars.next();
            } else {
                break;
            }
        }
        if end == frac_start {
            // "1." or "." with no digits after the point
            return Err(LexError {
                kind: LexErrorKind::MalformedNumber(src[start..end].to_string()),
                pos: start,
            });
        }
        // a second '.' directly after ("1.2.3") must not become two numbers
        if let Some(&(_, '.')) = chars.peek() {
            return Err(LexError {
                kind: LexErrorKind::MalformedNumber(src[start..end + 1].to_string()),
                pos: start,
            });
        }
    }
    Ok(end)
}