use std::fmt::Display;

use lace_lexer::token::Token;

use crate::ast::Expression;

// TODO: Make this more detailed. I have no idea how at this moment
pub trait ParserError {
    fn err_msg(&self) -> String;
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
    fn err_msg(&self) -> String {
        format!("No prefix parser found for {}", self.token)
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
            CondIssue::ExprIncorrectlyOpened => write!(f, "Conditional expression didn't open properly"),
            CondIssue::ExprIncorrectlyClosed => write!(f, "Condtional expression didn't close properly"),
            CondIssue::BodyIncorrectlyOpened => write!(f, "Body of conditonal expression didn't open properly"),
            CondIssue::ExpectedElse => write!(f, "Expected an Else block for this conditional expression"),
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
    fn err_msg(&self) -> String {
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
            FuncIssue::FuncMissingParens => write!(f, "Expected parentheses after function definition"),
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
    fn err_msg(&self) -> String {
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
    fn err_msg(&self) -> String {
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
    fn err_msg(&self) -> String {
        format!("Expected an identifier, received {}", self.found)
    }
}

pub struct ExpectedInteger {
    found: String,
}

impl From<String> for ExpectedInteger {
    fn from(value: String) -> Self {
        Self { found: value }
    }
}

impl ParserError for ExpectedInteger {
    fn err_msg(&self) -> String {
        format!("Expected an integer, received {}", self.found)
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
    fn err_msg(&self) -> String {
        format!("Expected {}, found {}", self.expected, self.got)
    }
}
