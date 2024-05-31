use std::{fmt::Display, usize};

use lace_lexer::token::{span::Span, Token};

use crate::ast::ExpressionKind;

// TODO: Make this more detailed. I have no idea how at this moment
pub trait ParserError {
    fn emit_err(&self) -> String;
    fn range(&self) -> (usize, usize) {
        (0, 0)
    }
    fn width(&self) -> (usize, usize) {
        (0, 0)
    }
    fn err_head(&self) -> String {
        "".into()
    }
    fn check_false_illegal(&self) -> bool {
        false
    }
}

pub struct NoPrefixParser {
    pub token: Token,
}

impl ParserError for NoPrefixParser {
    fn err_head(&self) -> String {
        format!(
            "\x1b[94m--> At {}:{}\x1b[0m",
            self.token.span.start_line,
            self.token.span.start_col + 1
        )
    }

    fn emit_err(&self) -> String {
        format!("\tEncountered an illegal token {}", self.token.kind)
    }

    fn range(&self) -> (usize, usize) {
        (self.token.span.start_line, self.token.span.end_line)
    }

    fn width(&self) -> (usize, usize) {
        (self.token.span.start_col, self.token.span.end_col)
    }

    fn check_false_illegal(&self) -> bool {
        self.token.is_actually_legal()
    }
}

pub enum CondIssue {
    ExprIncorrectlyOpened,
    ExprIncorrectlyClosed,
    BodyIncorrectlyOpened,
    ExpectedElse,
}

impl Display for CondIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CondIssue::ExprIncorrectlyOpened => {
                write!(
                    f,
                    "Conditional expression didn't open properly. Expected '('"
                )
            }
            CondIssue::ExprIncorrectlyClosed => {
                write!(
                    f,
                    "Condtional expression didn't close properly. Expected ')'"
                )
            }
            CondIssue::BodyIncorrectlyOpened => {
                write!(
                    f,
                    "Body of conditonal expression didn't open properly. Expected '{{'"
                )
            }
            CondIssue::ExpectedElse => {
                write!(f, "Expected an Else block for this conditional expression.")
            }
        }
    }
}

pub struct IncompleteConditional {
    pub start: Span,
    issue: CondIssue,
    pub end: Option<Span>,
    pub current: usize,
}

impl IncompleteConditional {
    pub fn new(start: Span, issue: CondIssue, end: Option<Span>, current: usize) -> Self {
        Self {
            start,
            issue,
            end,
            current,
        }
    }
}

macro_rules! check_end_line {
    ($e:expr) => {
        match &$e.end {
            Some(ln) => ln.end_line,
            None => $e.start.end_line,
        }
    };
}

macro_rules! check_end_col {
    ($e:expr) => {
        match &$e.end {
            Some(ln) => ln.end_col,
            None => $e.start.end_col,
        }
    };
}

impl ParserError for IncompleteConditional {
    fn emit_err(&self) -> String {
        format!("\t{}", self.issue)
    }

    fn range(&self) -> (usize, usize) {
        (self.start.start_line, check_end_line!(&self))
    }

    fn width(&self) -> (usize, usize) {
        match self.issue {
            CondIssue::ExprIncorrectlyOpened => (self.start.start_col - 1, check_end_col!(&self)),
            CondIssue::ExprIncorrectlyClosed => (self.start.start_col, check_end_col!(&self)),
            CondIssue::BodyIncorrectlyOpened => (self.start.start_col - 1, check_end_col!(&self)),
            CondIssue::ExpectedElse => (self.start.start_col - 1, check_end_col!(&self)),
        }
    }

    fn err_head(&self) -> String {
        format!("\x1b[94m--> At {}:{}\x1b[0m", self.range().1, self.current,)
    }
}

pub enum FuncIssue {
    FuncMissingParens,
    BodyIncorrectlyOpened,
    DefIncorrectlyClosed,
}

impl Display for FuncIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncIssue::FuncMissingParens => {
                write!(f, "Expected parentheses after function definition")
            }
            FuncIssue::BodyIncorrectlyOpened => write!(f, "Function body wasn't opened properly"),
            FuncIssue::DefIncorrectlyClosed => {
                write!(f, "Function definition wasn't closed properly")
            }
        }
    }
}

pub struct FuncError {
    pub start: Span,
    issue: FuncIssue,
    pub end: Option<Span>,
    pub current: usize,
}

impl FuncError {
    pub fn new(start: Span, issue: FuncIssue, end: Option<Span>, current: usize) -> Self {
        Self {
            start,
            issue,
            end,
            current,
        }
    }
}

impl ParserError for FuncError {
    fn emit_err(&self) -> String {
        format!("\t{}", self.issue)
    }

    fn range(&self) -> (usize, usize) {
        (self.start.start_line, check_end_line!(&self))
    }

    fn width(&self) -> (usize, usize) {
        match self.issue {
            FuncIssue::FuncMissingParens => (self.start.start_col - 1, check_end_col!(&self)),
            FuncIssue::BodyIncorrectlyOpened => {
                (self.start.start_col - 1, check_end_col!(&self) + 3)
            }
            FuncIssue::DefIncorrectlyClosed => (self.start.start_col - 1, check_end_col!(&self)),
        }
    }

    fn err_head(&self) -> String {
        format!("\x1b[94m--> At {}:{}\x1b[0m", self.range().1, self.current,)
    }
}

pub struct ExprError {
    expr: Option<ExpressionKind>,
}

impl From<Option<ExpressionKind>> for ExprError {
    fn from(value: Option<ExpressionKind>) -> Self {
        Self { expr: value }
    }
}

impl ParserError for ExprError {
    fn emit_err(&self) -> String {
        match &self.expr {
            Some(expr) => format!("{} Failed to parse expression", expr),
            None => "Expression did not close properly".into(),
        }
    }
}

pub struct ExpectedIdent {
    found: Token,
}

impl From<Token> for ExpectedIdent {
    fn from(value: Token) -> Self {
        Self { found: value }
    }
}

impl ParserError for ExpectedIdent {
    fn emit_err(&self) -> String {
        format!("Expected an identifier, received {}", self.found)
    }

    fn err_head(&self) -> String {
        format!(
            "\x1b[94m--> At {}:{}\x1b[0m",
            self.found.span.start_line,
            self.found.span.start_col + 1
        )
    }

    fn range(&self) -> (usize, usize) {
        (self.found.span.start_line, self.found.span.end_line)
    }

    fn width(&self) -> (usize, usize) {
        (self.found.span.start_col, self.found.span.end_col)
    }

    fn check_false_illegal(&self) -> bool {
        self.found.is_actually_legal()
    }
}

pub enum NumKind {
    Int,
    Float,
}

pub struct ExpectedNumber {
    kind: NumKind,
    found: String,
}

impl ExpectedNumber {
    pub fn new(kind: NumKind, found: String) -> Self {
        Self { kind, found }
    }
}

impl ParserError for ExpectedNumber {
    fn emit_err(&self) -> String {
        let kind = match self.kind {
            NumKind::Int => "an integer",
            NumKind::Float => "a character",
        };

        format!("Expected {}, received {}", kind, self.found)
    }
}

pub enum UnterminatedKind {
    Char,
    Str,
}

pub struct UnterminatedLiteral {
    kind: UnterminatedKind,
}

impl From<UnterminatedKind> for UnterminatedLiteral {
    fn from(value: UnterminatedKind) -> Self {
        Self { kind: value }
    }
}

impl ParserError for UnterminatedLiteral {
    fn emit_err(&self) -> String {
        match self.kind {
            UnterminatedKind::Char => "Unterminated character".into(),
            UnterminatedKind::Str => "Unterminated string".into(),
        }
    }
}

pub struct BadExpectations {
    expected: Token,
    got: Token,
}

impl BadExpectations {
    pub fn new(expected: Token, got: Token) -> Self {
        Self { expected, got }
    }
}

impl ParserError for BadExpectations {
    fn emit_err(&self) -> String {
        format!("Expected {}, found {}", self.expected, self.got)
    }
}
