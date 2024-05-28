use std::fmt::Display;

use lace_lexer::token::Token;

use crate::ast::Expression;

// TODO: Make this more detailed. I have no idea how at this moment
pub trait ParserError {
    fn log_err(&self) -> String;
}

pub struct NoPrefixParser {
    token: Token,
}

impl From<Token> for NoPrefixParser {
    fn from(value: Token) -> Self {
        Self { token: value }
    }
}

impl ParserError for NoPrefixParser {
    fn log_err(&self) -> String {
        format!(
            "At {}:{} => No prefix parser found for {}",
            self.token.span.start_line, self.token.span.start_col, self.token.kind
        )
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
                write!(f, "Conditional expression didn't open properly")
            }
            CondIssue::ExprIncorrectlyClosed => {
                write!(f, "Condtional expression didn't close properly")
            }
            CondIssue::BodyIncorrectlyOpened => {
                write!(f, "Body of conditonal expression didn't open properly")
            }
            CondIssue::ExpectedElse => {
                write!(f, "Expected an Else block for this conditional expression")
            }
        }
    }
}

pub struct IncompleteConditional {
    expr: Option<Expression>,
    issue: CondIssue,
}

impl IncompleteConditional {
    pub fn new(expr: Option<Expression>, issue: CondIssue) -> Self {
        Self { expr, issue }
    }
}

impl ParserError for IncompleteConditional {
    fn log_err(&self) -> String {
        match &self.expr {
            Some(expr) => format!("{}: {}", expr, self.issue),
            None => format!("{}", self.issue),
        }
    }
}

pub enum FuncIssue {
    FuncMissingParens,
    BodyIncorrectlyOpened,
    BodyIncorrectlyClosed,
}

impl Display for FuncIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncIssue::FuncMissingParens => {
                write!(f, "Expected parentheses after function definition")
            }
            FuncIssue::BodyIncorrectlyOpened => write!(f, "Function body wasn't opened properly"),
            FuncIssue::BodyIncorrectlyClosed => write!(f, "Function body wasn't closed properly"),
        }
    }
}

pub struct FuncError {
    func_name: Option<String>,
    issue: FuncIssue,
}

impl FuncError {
    pub fn new(func_name: Option<String>, issue: FuncIssue) -> Self {
        Self { func_name, issue }
    }
}

impl ParserError for FuncError {
    fn log_err(&self) -> String {
        match &self.func_name {
            Some(name) => format!("{}: {}", name, self.issue),
            None => format!("{}", self.issue),
        }
    }
}

pub struct ExprError {
    expr: Option<Expression>,
}

impl From<Option<Expression>> for ExprError {
    fn from(value: Option<Expression>) -> Self {
        Self { expr: value }
    }
}

impl ParserError for ExprError {
    fn log_err(&self) -> String {
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
    fn log_err(&self) -> String {
        format!("Expected an identifier, received {}", self.found)
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
    fn log_err(&self) -> String {
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
    fn log_err(&self) -> String {
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
    fn log_err(&self) -> String {
        format!("Expected {}, found {}", self.expected, self.got)
    }
}
