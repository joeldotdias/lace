pub mod kind;
pub mod span;

use std::fmt::Display;

use kind::TokenKind;
use span::{dummy_span, Span};

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
        self.kind() == other.kind()
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
}

pub fn dummy_token(kind: TokenKind) -> Token {
    Token {
        kind,
        span: dummy_span(),
    }
}
