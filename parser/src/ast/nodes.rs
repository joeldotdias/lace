use core::panic;
use std::fmt::Display;

use lace_lexer::token::{
    dummy_token,
    kind::{LiteralKind, TokenKind},
    span::Span,
    Token,
};

use crate::{
    ast::{statement::BlockStatement, ExpressionKind, Precedence},
    errors::{
        CondIssue, ExpectedIdent, ExpectedNumber, FuncError, FuncIssue, IncompleteConditional,
        NoPrefixParser, NumKind, ParserError, UnterminatedKind, UnterminatedLiteral,
    },
    Parser,
};

#[derive(PartialEq, Debug, Clone)]
pub struct IdentNode {
    pub token: Token,
    pub label: String,
}

impl Display for IdentNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident '{}'", &self.label)
    }
}

impl IdentNode {
    pub fn new(token: Token) -> Self {
        if let TokenKind::Ident { label } = &token.kind {
            Self {
                token: token.clone(),
                label: label.into(),
            }
        } else {
            panic!("This function shouldn't have been called");
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        if let TokenKind::Ident { label } = &parser.curr_token.kind {
            Ok(Self {
                token: parser.curr_token.clone(),
                label: label.into(),
            })
        } else {
            Err(Box::new(ExpectedIdent::from(parser.curr_token.clone())))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveNode {
    IntegerLiteral(i64),
    FloatLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),
    BooleanLiteral(bool),
}

impl Display for PrimitiveNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveNode::IntegerLiteral(val) => write!(f, "{} (Int)", val),
            PrimitiveNode::FloatLiteral(val) => write!(f, "{} (Float)", val),
            PrimitiveNode::CharLiteral(val) => write!(f, "'{}' (Char)", val),
            PrimitiveNode::StringLiteral(val) => write!(f, "\"{}\" (Str)", val),
            PrimitiveNode::BooleanLiteral(val) => write!(f, "{} (Bool)", val),
        }
    }
}

impl PrimitiveNode {
    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        match &parser.curr_token.kind {
            TokenKind::Literal { kind, val } => {
                match kind {
                    LiteralKind::Int => match val.parse::<i64>() {
                        Ok(val) => Ok(PrimitiveNode::IntegerLiteral(val)),
                        Err(_) => Err(Box::new(ExpectedNumber::new(NumKind::Int, val.to_string()))),
                    },
                    LiteralKind::Float => match val.parse::<f64>() {
                        Ok(val) => Ok(PrimitiveNode::FloatLiteral(val)),
                        Err(_) => Err(Box::new(ExpectedNumber::new(
                            NumKind::Float,
                            val.to_string(),
                        ))),
                    },
                    LiteralKind::Char { terminated } => match terminated {
                        // TODO: Maybe find a better way to do this
                        true => Ok(PrimitiveNode::CharLiteral(val.chars().nth(0).unwrap())),
                        false => Err(Box::new(UnterminatedLiteral::from(UnterminatedKind::Char))),
                    },
                    LiteralKind::Str { terminated } => match terminated {
                        true => Ok(PrimitiveNode::StringLiteral(val.into())),
                        false => Err(Box::new(UnterminatedLiteral::from(UnterminatedKind::Str))),
                    },
                }
            }
            TokenKind::True => Ok(PrimitiveNode::BooleanLiteral(true)),
            TokenKind::False => Ok(PrimitiveNode::BooleanLiteral(false)),
            _ => Err(Box::new(NoPrefixParser {
                token: parser.curr_token.clone(),
            })),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrefixOperator {
    pub operator: Token,
    pub right_expr: Box<ExpressionKind>,
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prefix({}){}", self.operator, self.right_expr)
    }
}

impl PrefixOperator {
    pub fn new(token: Token, right: ExpressionKind) -> Self {
        Self {
            operator: token,
            right_expr: Box::new(right),
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        let token = parser.curr_token.clone();
        parser.next_token();
        let right = ExpressionKind::parse(parser, Precedence::Prefix)?;

        Ok(PrefixOperator::new(token, right))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct InfixOperator {
    pub operator: Token,
    pub left_expr: Box<ExpressionKind>,
    pub right_expr: Box<ExpressionKind>,
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Infix({}) {}",
            self.left_expr, self.operator, self.right_expr
        )
    }
}

impl InfixOperator {
    pub fn new(token: Token, left_expr: ExpressionKind, right_expr: ExpressionKind) -> Self {
        Self {
            operator: token,
            left_expr: Box::new(left_expr),
            right_expr: Box::new(right_expr),
        }
    }

    pub fn parse(
        parser: &mut Parser,
        left_expr: ExpressionKind,
    ) -> Result<Self, Box<dyn ParserError>> {
        let token = parser.curr_token.clone();
        let precedence = parser.curr_precedence();

        parser.next_token();

        let right_expr = ExpressionKind::parse(parser, precedence)?;

        Ok(InfixOperator::new(token, left_expr, right_expr))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalOperator {
    pub cond: Box<ExpressionKind>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Display for ConditionalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut expr = format!(
            "Conditonal => {{ condition => {} | consequence => {{\n{}}}",
            self.cond, self.consequence,
        );

        if let Some(alt) = &self.alternative {
            expr.push_str(format!(" | alternative => {{\n{}}}", alt).as_str())
        }

        expr.push_str(" \n}}");

        write!(f, "{}", expr)
    }
}

impl ConditionalOperator {
    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        let start_pos = parser.lexer.curr_pos();
        if !parser.expect_peek(&dummy_token(TokenKind::LParen)) {
            return Err(Box::new(IncompleteConditional::new(
                start_pos,
                CondIssue::ExprIncorrectlyOpened,
                None,
                parser.lexer.curr_col(),
            )));
        }

        parser.next_token();
        let cond = ExpressionKind::parse(parser, Precedence::Lowest)?;

        if !parser.expect_peek(&dummy_token(TokenKind::RParen)) {
            return Err(Box::new(IncompleteConditional::new(
                start_pos,
                CondIssue::ExprIncorrectlyClosed,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col() - 1,
            )));
        }

        if !parser.expect_peek(&dummy_token(TokenKind::LCurly)) {
            return Err(Box::new(IncompleteConditional::new(
                start_pos,
                CondIssue::BodyIncorrectlyOpened,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col(),
            )));
        }

        let consequence = BlockStatement::parse(parser);
        let mut alternative = None;

        if parser.peek_token_is(&TokenKind::Else) {
            parser.next_token();
            if !parser.expect_peek(&dummy_token(TokenKind::LCurly)) {
                return Err(Box::new(IncompleteConditional::new(
                    start_pos,
                    CondIssue::ExpectedElse,
                    Some(parser.lexer.curr_pos()),
                    parser.lexer.curr_col(),
                )));
            }

            alternative = Some(BlockStatement::parse(parser));
        }

        Ok(ConditionalOperator {
            cond: Box::new(cond),
            consequence,
            alternative,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionLiteral {
    pub name: Option<String>,
    pub params: Vec<IdentNode>,
    pub body: BlockStatement,
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.params.iter().map(ToString::to_string).collect();

        match &self.name {
            Some(name) => write!(
                f,
                "\nFunc => {{ Name => {} | Params => ({}) | Body => {{\n{}}}\n",
                name,
                params.join(", "),
                self.body
            ),
            None => write!(
                f,
                "Func\n--> Params => ({})\n-->{{\n{}}}",
                params.join(", "),
                self.body
            ),
        }
    }
}

impl FunctionLiteral {
    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        let start_pos = parser.lexer.curr_pos();
        let name = Self::parse_function_name(parser);

        if !parser.expect_peek(&dummy_token(TokenKind::LParen)) {
            return Err(Box::new(FuncError::new(
                start_pos,
                FuncIssue::FuncMissingParens,
                None,
                parser.lexer.curr_col(),
            )));
        }

        let params = Self::parse_function_params(parser, &start_pos)?;

        if !parser.expect_peek(&dummy_token(TokenKind::LCurly)) {
            return Err(Box::new(FuncError::new(
                start_pos,
                FuncIssue::BodyIncorrectlyOpened,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col(),
            )));
        }

        let body = BlockStatement::parse(parser);

        Ok(FunctionLiteral { name, params, body })
    }

    fn parse_function_name(parser: &mut Parser) -> Option<String> {
        let mut name = None;

        if let TokenKind::Ident { label } = &parser.peeked_token.kind {
            name = Some(label.to_string());
            parser.next_token();
        };

        name
    }

    fn parse_function_params(
        parser: &mut Parser,
        start: &Span,
    ) -> Result<Vec<IdentNode>, Box<dyn ParserError>> {
        let mut idents = Vec::<IdentNode>::new();

        if parser.peek_token_is(&TokenKind::RParen) {
            parser.next_token();
            return Ok(idents);
        }

        parser.next_token();

        let mut ident = IdentNode::new(parser.curr_token.clone());
        idents.push(ident);

        while parser.peek_token_is(&TokenKind::Comma) {
            parser.next_token(); // skip the comma
            parser.next_token(); // capture the param

            ident = IdentNode::new(parser.curr_token.clone());
            idents.push(ident);
        }

        if !parser.expect_peek(&dummy_token(TokenKind::RParen)) {
            return Err(Box::new(FuncError::new(
                start.clone(),
                FuncIssue::BodyIncorrectlyClosed,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col(),
            )));
        }

        Ok(idents)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub function: Box<ExpressionKind>,
    pub args: Vec<ExpressionKind>,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: Vec<String> = self.args.iter().map(ToString::to_string).collect();

        write!(
            f,
            "Fn Call {} => params {{ {} }}",
            self.function,
            args.join(", ")
        )
    }
}

impl FunctionCall {
    pub fn parse(
        parser: &mut Parser,
        function: ExpressionKind,
    ) -> Result<Self, Box<dyn ParserError>> {
        let args = ExpressionKind::parse_expr_list(parser, &dummy_token(TokenKind::RParen))?;

        Ok(FunctionCall {
            function: Box::new(function),
            args,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<ExpressionKind>,
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        write!(f, "Array => {{ Elements => [ {} ] }}", elements.join(", "))
    }
}

impl ArrayLiteral {
    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        let elements = ExpressionKind::parse_expr_list(parser, &dummy_token(TokenKind::RBracket))?;
        Ok(Self { elements })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IndexAccess {
    pub arr: Box<ExpressionKind>,
    pub index: Box<ExpressionKind>,
}

impl Display for IndexAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array => {} | Index => {}", self.arr, self.index)
    }
}
impl IndexAccess {
    pub fn parse(
        parser: &mut Parser,
        left_expr: ExpressionKind,
    ) -> Result<Self, Box<dyn ParserError>> {
        parser.next_token();
        let index = ExpressionKind::parse(parser, Precedence::Lowest)?;
        if !parser.expect_peek(&dummy_token(TokenKind::RBracket)) {
            // TODO: this does not belong here. Change it
            // return Err(Box::new(FuncError::new(
            //     None,
            //     FuncIssue::BodyIncorrectlyClosed,
            // )));
        }

        Ok(IndexAccess {
            arr: Box::new(left_expr),
            index: Box::new(index),
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct HashLiteral {
    pub pairs: Vec<(ExpressionKind, ExpressionKind)>,
}

impl Display for HashLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = self
            .pairs
            .iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>();

        write!(f, "{{ {} }}", pairs.join(","))
    }
}

impl HashLiteral {
    pub fn parse(parser: &mut Parser) -> Result<Self, Box<dyn ParserError>> {
        let mut pairs = Vec::new();

        while !parser.peek_token_is(&TokenKind::RCurly) {
            parser.next_token();
            let key = ExpressionKind::parse(parser, Precedence::Lowest)?;
            if !parser.expect_peek(&dummy_token(TokenKind::Colon)) {
                // TODO: this does not belong here. Change it
                // return Err(Box::new(FuncError::new(
                //     None,
                //     FuncIssue::BodyIncorrectlyClosed,
                // )));
            }

            parser.next_token();
            let val = ExpressionKind::parse(parser, Precedence::Lowest)?;

            pairs.push((key, val));

            if !parser.peek_token_is(&TokenKind::RCurly)
                && !parser.expect_peek(&dummy_token(TokenKind::Comma))
            {
                // TODO: this does not belong here. Change it
                // return Err(Box::new(FuncError::new(
                //     None,
                //     FuncIssue::BodyIncorrectlyClosed,
                // )));
            }
        }

        if !parser.expect_peek(&dummy_token(TokenKind::RCurly)) {
            // return Err(Box::new(FuncError::new(
            //     None,
            //     FuncIssue::BodyIncorrectlyClosed,
            // )));
        }

        Ok(HashLiteral { pairs })
    }
}
