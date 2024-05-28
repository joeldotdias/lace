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

use self::nodes::{ArrayLiteral, HashLiteral, IndexAccess};

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
    Unary(PrefixOperator),
    Binary(InfixOperator),
    Conditional(ConditionalOperator),
    FunctionDef(FunctionLiteral),
    FunctionCall(FunctionCall),
    Array(ArrayLiteral),
    ArrIndex(IndexAccess),
    HashMapLiteral(HashLiteral),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(x) => write!(f, "{x}"),
            Expression::Primitive(x) => write!(f, "{x}"),
            Expression::Unary(x) => write!(f, "{x}"),
            Expression::Binary(x) => write!(f, "{x}"),
            Expression::Conditional(x) => write!(f, "{x}"),
            Expression::FunctionDef(x) => write!(f, "{x}"),
            Expression::FunctionCall(x) => write!(f, "{x}"),
            Expression::Array(x) => write!(f, "{}", x),
            Expression::ArrIndex(x) => write!(f, "{}", x),
            Expression::HashMapLiteral(x) => write!(f, "{}", x),
        }
    }
}

impl Expression {
    pub fn parse(
        parser: &mut Parser,
        precedence: Precedence,
    ) -> Result<Expression, Box<dyn ParserError>> {
        let mut left_expr = match parser.curr_token.clone() {
            Token::Ident { label: _ } => IdentNode::parse(parser).map(Expression::Identifier),
            Token::Literal { kind: _, val: _ } | Token::False | Token::True => {
                PrimitiveNode::parse(parser).map(Expression::Primitive)
            }
            Token::Bang | Token::Minus => PrefixOperator::parse(parser).map(Expression::Unary),
            Token::LParen => Self::parse_grouped_expr(parser),
            Token::If => ConditionalOperator::parse(parser).map(Expression::Conditional),
            Token::Function => FunctionLiteral::parse(parser).map(Expression::FunctionDef),
            Token::LBracket => ArrayLiteral::parse(parser).map(Expression::Array),
            Token::LCurly => HashLiteral::parse(parser).map(Expression::HashMapLiteral),
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
                    left_expr = Expression::Binary(InfixOperator::parse(parser, left_expr)?);
                }

                Token::LParen => {
                    parser.next_token();
                    left_expr = Expression::FunctionCall(FunctionCall::parse(parser, left_expr)?);
                }

                Token::LBracket => {
                    parser.next_token();
                    left_expr = Expression::ArrIndex(IndexAccess::parse(parser, left_expr)?);
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
    Comparative = 2,
    Additive = 3,
    Multiplicative = 4,
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
            Token::Plus | Token::Minus | Token::Or => Precedence::Additive,
            Token::ForwardSlash | Token::Asterisk | Token::Modulo | Token::And => {
                Precedence::Multiplicative
            }
            Token::LParen => Precedence::FnCall,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
