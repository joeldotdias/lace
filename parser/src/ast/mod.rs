pub mod nodes;
pub mod statement;

use std::fmt::Display;

use lace_lexer::token::{dummy_token, kind::TokenKind, span::dummy_span, Token};
use nodes::{
    ArrayLiteral, ConditionalOperator, FunctionCall, FunctionLiteral, HashLiteral, IdentNode,
    IndexAccess, InfixOperator, PrefixOperator, PrimitiveNode,
};
use statement::Statement;

use crate::{
    errors::{ExprError, NoPrefixParser, ParserError},
    Parser,
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
        let mut left_expr = match &parser.curr_token.kind {
            TokenKind::Ident { label: _ } => IdentNode::parse(parser).map(Expression::Identifier),
            TokenKind::Literal { kind: _, val: _ } | TokenKind::False | TokenKind::True => {
                PrimitiveNode::parse(parser).map(Expression::Primitive)
            }
            TokenKind::Bang | TokenKind::Minus => {
                PrefixOperator::parse(parser).map(Expression::Unary)
            }
            TokenKind::LParen => Self::parse_grouped_expr(parser),
            TokenKind::If => ConditionalOperator::parse(parser).map(Expression::Conditional),
            TokenKind::Function => FunctionLiteral::parse(parser).map(Expression::FunctionDef),
            TokenKind::LBracket => ArrayLiteral::parse(parser).map(Expression::Array),
            TokenKind::LCurly => HashLiteral::parse(parser).map(Expression::HashMapLiteral),
            _ => {
                return Err(Box::new(NoPrefixParser {
                    token: parser.curr_token.clone(),
                }))
            }
        }?;

        while !parser.peek_token_is(&TokenKind::Semicolon) && precedence < parser.peek_precedence()
        {
            match &parser.peeked_token.kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::ForwardSlash
                | TokenKind::Asterisk
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::GreaterThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThanEqual
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Modulo => {
                    parser.next_token();
                    left_expr = Expression::Binary(InfixOperator::parse(parser, left_expr)?);
                }

                TokenKind::LParen => {
                    parser.next_token();
                    left_expr = Expression::FunctionCall(FunctionCall::parse(parser, left_expr)?);
                }

                TokenKind::LBracket => {
                    parser.next_token();
                    left_expr = Expression::ArrIndex(IndexAccess::parse(parser, left_expr)?);
                }

                _ => return Ok(left_expr),
            }
        }

        Ok(left_expr)
    }

    fn parse_grouped_expr(parser: &mut Parser) -> Result<Expression, Box<dyn ParserError>> {
        let start = parser.lexer.curr_pos();
        parser.next_token();

        let expr = Expression::parse(parser, Precedence::Lowest);

        if parser.expect_peek(&dummy_token(TokenKind::RParen)) {
            expr
        } else {
            Err(Box::new(ExprError::new(
                start,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col(),
            )))
        }
    }

    pub fn parse_expr_list(
        parser: &mut Parser,
        end: &Token,
    ) -> Result<Vec<Expression>, Box<dyn ParserError>> {
        let mut expr_list = Vec::new();
        if parser.peek_token_is(&end.kind) {
            parser.next_token();
            return Ok(expr_list);
        }

        parser.next_token();
        expr_list.push(Expression::parse(parser, Precedence::Lowest)?);
        let mut start = dummy_span();

        while parser.peek_token_is(&TokenKind::Comma) {
            parser.next_token();
            start = parser.lexer.curr_pos();
            parser.next_token();
            expr_list.push(Expression::parse(parser, Precedence::Lowest)?);
        }

        if !parser.expect_peek(end) {
            Err(Box::new(ExprError::new(
                start,
                Some(parser.lexer.curr_pos()),
                parser.lexer.curr_col(),
            )))
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
        match value.kind {
            TokenKind::Equal | TokenKind::NotEqual => Precedence::Equality,
            TokenKind::LessThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEqual => Precedence::Index,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Or => Precedence::Additive,
            TokenKind::ForwardSlash | TokenKind::Asterisk | TokenKind::Modulo | TokenKind::And => {
                Precedence::Multiplicative
            }
            TokenKind::LParen => Precedence::FnCall,
            TokenKind::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
