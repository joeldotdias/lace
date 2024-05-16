pub mod nodes;
pub mod statement;

use std::fmt::Display;

use nodes::{
    ConditionalOperator, FunctionCall, FunctionLiteral, IdentNode, InfixOperator, PrefixOperator,
    PrimitiveNode,
};
use statement::Statement;

use crate::{
    errors::{ExprError, NoPrefixParser, ParserError},
    Parser, Token,
};

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pstr = String::new();

        for statement in &self.statements {
            pstr.push_str(format!("{}\n", statement).as_str());
        }

        write!(f, "{}", pstr)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(IdentNode),
    Primitive(PrimitiveNode),
    Prefix(PrefixOperator),
    Infix(InfixOperator),
    Conditional(ConditionalOperator),
    FunctionDef(FunctionLiteral),
    FunctionCall(FunctionCall),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(x) => write!(f, "{x}"),
            Expression::Primitive(x) => write!(f, "{x}"),
            Expression::Prefix(x) => write!(f, "{x}"),
            Expression::Infix(x) => write!(f, "{x}"),
            Expression::Conditional(x) => write!(f, "{x}"),
            Expression::FunctionDef(x) => write!(f, "{x}"),
            Expression::FunctionCall(x) => write!(f, "{x}"),
        }
    }
}

impl Expression {
    pub fn parse(
        parser: &mut Parser,
        precedence: Precedence,
    ) -> Result<Expression, Box<dyn ParserError>> {
        let mut left_expr = match parser.curr_token.clone() {
            Token::Ident(_) => (IdentNode::parse(parser)).map(Expression::Identifier),
            Token::Literal { kind: _, val: _ } | Token::False | Token::True => {
                PrimitiveNode::parse(parser).map(Expression::Primitive)
            }
            Token::Bang | Token::Minus => PrefixOperator::parse(parser).map(Expression::Prefix),
            Token::LParen => Self::parse_grouped_expr(parser),
            Token::If => ConditionalOperator::parse(parser).map(Expression::Conditional),
            Token::Function => FunctionLiteral::parse(parser).map(Expression::FunctionDef),
            _ => return Err(Box::new(NoPrefixParser::from(parser.curr_token.clone()))),
        }?;

        while !parser.peek_token_is(&Token::Semicolon) && precedence < parser.peek_precedence() {
            match &parser.peeked_token {
                Token::Plus
                | Token::Minus
                | Token::ForwardSlash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::GreaterThan
                | Token::LessThanEqual
                | Token::GreaterThanEqual
                | Token::And
                | Token::Or
                | Token::Modulo => {
                    parser.next_token();
                    left_expr = Expression::Infix(InfixOperator::parse(parser, left_expr)?);
                }

                Token::LParen => {
                    parser.next_token();
                    left_expr = Expression::FunctionCall(FunctionCall::parse(parser, left_expr)?);
                }

                _ => return Ok(left_expr),
            }
        }

        Ok(left_expr)
    }

    fn parse_grouped_expr(parser: &mut Parser) -> Result<Expression, Box<dyn ParserError>> {
        parser.next_token();

        let expr = Expression::parse(parser, Precedence::Lowest);

        if parser.expect_peek(&Token::RParen) {
            expr
        } else {
            Err(Box::new(ExprError::from(None)))
        }
    }

    pub fn parse_expr_list(
        parser: &mut Parser,
        end: &Token,
    ) -> Result<Vec<Expression>, Box<dyn ParserError>> {
        let mut expr_list = Vec::new();
        if parser.peek_token_is(end) {
            parser.next_token();
            return Ok(expr_list);
        }

        parser.next_token();
        expr_list.push(Expression::parse(parser, Precedence::Lowest)?);

        while parser.peek_token_is(&Token::Comma) {
            parser.next_token();
            parser.next_token();
            expr_list.push(Expression::parse(parser, Precedence::Lowest)?);
        }

        if !parser.expect_peek(end) {
            Err(Box::new(ExprError::from(None)))
        } else {
            Ok(expr_list)
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Precedence {
    Lowest = 0,
    Equality = 1,
    LessOrGreaterThan = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    FnCall = 6,
    Index = 7,
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::Equal | Token::NotEqual => Precedence::Equality,
            Token::LessThan
            | Token::LessThanEqual
            | Token::GreaterThan
            | Token::GreaterThanEqual => Precedence::Index,
            Token::Plus | Token::Minus | Token::Or => Precedence::Sum,
            Token::ForwardSlash | Token::Asterisk | Token::Modulo | Token::And => {
                Precedence::Product
            }
            Token::LParen => Precedence::FnCall,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
