use lexer::{token::Token, Lexer};

use crate::{
    ast::{Expression, Precedence, Program},
    nodes::IdentNode,
    statement::{LetStatement, ReturnStatement, Statement},
};

pub mod ast;
pub mod nodes;
pub mod statement;

#[cfg(test)]
mod tests;

pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: Token,
    pub peeked_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            curr_token: Token::Illegal,
            peeked_token: Token::Illegal,
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

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
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
        match self.curr_token {
            Token::Let => self.parse_let().map(Statement::Let),
            Token::Return => self.parse_return().map(Statement::Return),
            _ => self.parse_expression().map(Statement::Expr),
        }
    }

    pub fn parse_let(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(&Token::Ident(String::new())) {
            return None;
        }

        let name = match self.curr_token.clone() {
            Token::Ident(val) => IdentNode {
                token: self.curr_token.clone(),
                val,
            },
            _ => unreachable!("no happen please"),
        };

        if !self.expect_peek(&Token::Assign) {
            return None;
        }

        self.next_token();

        let mut val = match Expression::parse(self, Precedence::Lowest) {
            Ok(val) => {
                println!("Hello val :{}", val);
                val
            }
            Err(err) => {
                self.errors.push(err);
                return None;
            }
        };

        if let Expression::FunctionDef(literal) = &mut val {
            literal.name = Some(name.token.to_string());
        };

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(LetStatement { name, val })
    }

    fn parse_return(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        let return_val = match Expression::parse(self, Precedence::Lowest) {
            Ok(val) => val,
            Err(err) => {
                self.errors.push(err);
                return None;
            }
        };

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(ReturnStatement {
            returnable: return_val,
        })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let expr = Expression::parse(self, Precedence::Lowest);

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        match expr {
            Ok(expr) => Some(expr),
            Err(err) => {
                self.errors.push(err);
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
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_token_is(&mut self, token: &Token) -> bool {
        match self.peeked_token {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Literal { kind: _, val: _ } => {
                matches!(token, Token::Literal { kind: _, val: _ })
            }
            _ => &self.peeked_token == token,
        }
    }

    fn peek_error(&mut self, token: &Token) {
        self.errors
            .push(format!("Expected {}, got {}", token, self.peeked_token))
    }

    fn curr_token_is(&self, token: &Token) -> bool {
        match self.curr_token {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Literal { kind: _, val: _ } => {
                matches!(token, Token::Literal { kind: _, val: _ })
            }
            _ => &self.curr_token == token,
        }
    }
}