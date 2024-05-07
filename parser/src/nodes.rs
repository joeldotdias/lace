use core::panic;
use std::fmt::Display;

use lexer::token::{LiteralType, Token};

use crate::{
    ast::{Expression, Precedence},
    statement::BlockStatement,
    Parser,
};

#[derive(PartialEq, Debug, Clone)]
pub struct IdentNode {
    pub token: Token,
    pub val: String,
}

impl Display for IdentNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.val)
    }
}

impl IdentNode {
    pub fn new(token: Token) -> Self {
        if let Token::Ident(val) = token.clone() {
            Self { token, val }
        } else {
            panic!("This function shouldn't have been called");
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        if let Token::Ident(val) = &parser.curr_token {
            Ok(Self {
                token: parser.curr_token.clone(),
                val: val.into(),
            })
        } else {
            Err(format!(
                "Expected an identifier, received {}",
                parser.curr_token
            ))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum PrimitiveNode {
    IntegerLiteral(i64),
    StringLiteral(String),
    BooleanLiteral(bool),
}

impl Display for PrimitiveNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveNode::IntegerLiteral(val) => write!(f, "{}", val),
            PrimitiveNode::StringLiteral(val) => write!(f, "\"{}\"", val),
            PrimitiveNode::BooleanLiteral(val) => write!(f, "{}", val),
        }
    }
}

impl PrimitiveNode {
    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        match parser.curr_token.clone() {
            Token::Literal { kind, val } => {
                println!("val is: {}", val);
                match kind {
                    LiteralType::Int => match val.parse::<i64>() {
                        Ok(val) => Ok(PrimitiveNode::IntegerLiteral(val)),
                        Err(err) => Err(err.to_string()),
                    },
                    LiteralType::Str => Ok(PrimitiveNode::StringLiteral(val)),
                }
            }
            Token::True => Ok(PrimitiveNode::BooleanLiteral(true)),
            Token::False => Ok(PrimitiveNode::BooleanLiteral(false)),
            _ => Err(format!("{} is not primitive", parser.curr_token)),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrefixOperator {
    pub token: Token,
    pub right_expr: Box<Expression>,
}

impl PrefixOperator {
    pub fn new(token: Token, right: Expression) -> Self {
        Self {
            token,
            right_expr: Box::new(right),
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        let token = parser.curr_token.clone();
        parser.next_token();
        let right = Expression::parse(parser, Precedence::Prefix)?;

        Ok(PrefixOperator::new(token, right))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct InfixOperator {
    pub token: Token,
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}, {}", self.left_expr, self.token, self.right_expr)
    }
}

impl InfixOperator {
    pub fn new(token: Token, left_expr: Expression, right_expr: Expression) -> Self {
        Self {
            token,
            left_expr: Box::new(left_expr),
            right_expr: Box::new(right_expr),
        }
    }

    pub fn parse(parser: &mut Parser, left_expr: Expression) -> Result<Self, String> {
        let token = parser.curr_token.clone();
        let precedence = parser.curr_precedence();

        parser.next_token();

        let right_expr = Expression::parse(parser, precedence)?;

        Ok(InfixOperator::new(token, left_expr, right_expr))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConditionalOperator {
    pub cond: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Display for ConditionalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut expr = format!("if {}{{\n{}}}", self.cond, self.consequence,);

        if let Some(alt) = &self.alternative {
            expr.push_str(format!(" else {{\n{}}}", alt).as_str())
        }

        write!(f, "{}", expr)
    }
}

impl ConditionalOperator {
    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        if !parser.expect_peek(&Token::LParen) {
            return Err("Conditonal expression didn't open properly".into());
        }

        parser.next_token();
        let cond = Expression::parse(parser, Precedence::Lowest)?;

        if !parser.expect_peek(&Token::RParen) {
            return Err("Conditonal expression didn't close properly".into());
        }

        if !parser.expect_peek(&Token::LCurly) {
            return Err("Conditonal expression body didn't open properly".into());
        }

        let consequence = BlockStatement::parse(parser);
        let mut alternative = None;

        if parser.peek_token_is(&Token::Else) {
            parser.next_token();
            if !parser.expect_peek(&Token::LCurly) {
                return Err("Conditonal else expression didn't open properly".into());
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
            Some(name) => write!(f, "fn {}({}), {{\n{}}}", name, params.join(", "), self.body),
            None => write!(f, "fn ({}), {{\n{}}}", params.join(", "), self.body),
        }
    }
}

impl FunctionLiteral {
    pub fn parse(parser: &mut Parser) -> Result<Self, String> {
        if !parser.expect_peek(&Token::LParen) {
            return Err("No params".into());
        }

        let params = match Self::parse_function_params(parser) {
            Ok(params) => params,
            Err(err) => {
                return Err(err);
            }
        };

        if !parser.expect_peek(&Token::LCurly) {
            return Err("function body not opened".into());
        }

        let body = BlockStatement::parse(parser);

        Ok(FunctionLiteral {
            name: None,
            params,
            body,
        })
    }

    fn parse_function_params(parser: &mut Parser) -> Result<Vec<IdentNode>, String> {
        let mut idents = Vec::<IdentNode>::new();

        if parser.peek_token_is(&Token::RParen) {
            parser.next_token();
            return Ok(idents);
        }

        parser.next_token();

        let mut ident = IdentNode::new(parser.curr_token.clone());
        idents.push(ident);

        while parser.peek_token_is(&Token::Comma) {
            parser.next_token(); // skip the comma
            parser.next_token(); // capture the param

            ident = IdentNode::new(parser.curr_token.clone());
            idents.push(ident);
        }

        if !parser.expect_peek(&Token::RParen) {
            return Err("function wasn't closed properly".into());
        }

        Ok(idents)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub args: Vec<Expression>,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: Vec<String> = self.args.iter().map(ToString::to_string).collect();

        write!(f, "{}({})", self.function, args.join(", "))
    }
}

impl FunctionCall {
    pub fn parse(parser: &mut Parser, function: Expression) -> Result<Self, String> {
        let args = Expression::parse_expr_list(parser, &Token::RParen)?;

        Ok(FunctionCall {
            function: Box::new(function),
            args,
        })
    }
}
