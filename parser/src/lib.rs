use std::path::PathBuf;

use errors::{BadExpectations, ParserError};
use lace_lexer::{
    token::{dummy_token, kind::TokenKind, Token},
    Lexer,
};

use crate::ast::{
    nodes::IdentNode,
    statement::{LetStatement, ReturnStatement, SourceStatement, Statement},
    Expression, Precedence, Program,
};

pub mod ast;
pub mod errors;

#[cfg(test)]
mod tests;

pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: Token,
    pub peeked_token: Token,
    pub errors: Vec<Box<dyn ParserError>>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            curr_token: dummy_token(TokenKind::Eof),
            peeked_token: dummy_token(TokenKind::Eof),
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peeked_token.clone();
        self.peeked_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while !self.curr_token.reached_eof() {
            if let Some(statememt) = self.parse_statement() {
                program.statements.push(statememt);
            }

            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.kind {
            TokenKind::Let => self.parse_let().map(Statement::Assignment),
            TokenKind::Return => self.parse_return().map(Statement::Return),
            TokenKind::Source => self.parse_source().map(Statement::Source),
            _ => self.parse_expression().map(Statement::Expr),
        }
    }

    pub fn parse_let(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(&dummy_token(TokenKind::Ident {
            label: String::new(),
        })) {
            return None;
        }

        let name = match &self.curr_token.kind {
            TokenKind::Ident { label } => IdentNode {
                token: self.curr_token.clone(),
                label: label.into(),
            },
            _ => unreachable!("no happen please"),
        };

        if !self.expect_peek(&dummy_token(TokenKind::Assign)) {
            return None;
        }

        self.next_token();

        let mut val = match Expression::parse(self, Precedence::Lowest) {
            Ok(val) => val,
            Err(err) => {
                self.found_err(err);
                return None;
            }
        };

        if let Expression::FunctionDef(literal) = &mut val {
            literal.name = Some(name.token.to_string());
        };

        if self.peek_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        Some(LetStatement { name, val })
    }

    fn parse_return(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        let return_val = match Expression::parse(self, Precedence::Lowest) {
            Ok(val) => val,
            Err(err) => {
                self.found_err(err);
                return None;
            }
        };

        if self.peek_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        Some(ReturnStatement {
            returnable: return_val,
        })
    }

    fn parse_source(&mut self) -> Option<SourceStatement> {
        self.next_token();

        let sourceable = match Expression::parse(self, Precedence::Lowest) {
            Ok(e) => match e {
                Expression::Primitive(p) => match p {
                    ast::nodes::PrimitiveNode::IntegerLiteral(_) => todo!(),
                    ast::nodes::PrimitiveNode::FloatLiteral(_) => todo!(),
                    ast::nodes::PrimitiveNode::CharLiteral(_) => todo!(),
                    ast::nodes::PrimitiveNode::StringLiteral(s) => s,
                    ast::nodes::PrimitiveNode::BooleanLiteral(_) => todo!(),
                },
                _ => todo!(),
            },
            Err(err) => {
                self.found_err(err);
                return None;
            }
        };

        if self.peek_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        let path = PathBuf::from(&sourceable);

        Some(SourceStatement { path })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let expr = Expression::parse(self, Precedence::Lowest);

        if self.peek_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        match expr {
            Ok(expr) => Some(expr),
            Err(err) => {
                self.found_err(err);
                None
            }
        }
    }

    fn curr_precedence(&self) -> Precedence {
        Precedence::from(&self.curr_token)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(&self.peeked_token)
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token_is(&token.kind) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_token_is(&mut self, token_kind: &TokenKind) -> bool {
        match self.peeked_token.kind {
            TokenKind::Ident { label: _ } => matches!(token_kind, TokenKind::Ident { label: _ }),
            TokenKind::Literal { kind: _, val: _ } => {
                matches!(token_kind, TokenKind::Literal { kind: _, val: _ })
            }
            _ => &self.peeked_token.kind == token_kind,
        }
    }

    fn peek_error(&mut self, token: &Token) {
        self.found_err(Box::new(BadExpectations::new(
            token.clone(),
            self.peeked_token.clone(),
        )))
    }

    pub fn log_errors(&self) {
        self.errors.iter().for_each(|err| {
            println!("{}", err.emit_err());
        })
    }

    fn curr_token_is(&self, token: &Token) -> bool {
        match self.curr_token.kind {
            TokenKind::Ident { label: _ } => matches!(token.kind, TokenKind::Ident { label: _ }),
            TokenKind::Literal { kind: _, val: _ } => {
                matches!(token.kind, TokenKind::Literal { kind: _, val: _ })
            }
            _ => self.curr_token.kind == token.kind,
        }
    }

    fn found_err(&mut self, err: Box<dyn ParserError>) {
        self.errors.push(err);
    }
}
