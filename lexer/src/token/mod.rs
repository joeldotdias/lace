pub mod kind;
pub mod span;

use core::panic;
use std::fmt::Display;

use kind::TokenKind;
use span::{dummy_span, Span};

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.kind, self.span)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if let (TokenKind::Illegal { ch: _ }, TokenKind::Illegal { ch: _ }) =
            (self.kind(), other.kind())
        {
            true
        } else {
            self.kind() == other.kind()
        }
    }
}
impl Eq for Token {}

impl Token {
    fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn reached_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }

    pub fn is_actually_legal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::LBracket
                | TokenKind::RBracket
                | TokenKind::LParen
                | TokenKind::RParen
                | TokenKind::LCurly
                | TokenKind::RCurly
                | TokenKind::Semicolon
        )
    }
}

pub fn dummy_token(kind: TokenKind) -> Token {
    Token {
        kind,
        span: dummy_span(),
    }
}
