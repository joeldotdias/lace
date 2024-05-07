use std::fmt::Display;

use crate::{lexer::token::Token, parser::{ast::Expression, nodes::IdentNode}};

use super::Parser;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expr(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(statement) => write!(f, "{}", statement),
            Statement::Return(statement) => write!(f, "{}", statement),
            Statement::Expr(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LetStatement {
    pub name: IdentNode,
    pub val: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.val)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ReturnStatement {
    pub returnable: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.returnable)
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut block = String::new();
        self.statements.iter().for_each(|statement| {
            block.push_str(format!("{}\n",statement).as_str());
        });
        write!(f, "{}", block)
    }
}

impl BlockStatement {
    pub fn parse(parser: &mut Parser) -> Self {
        parser.next_token();

        let mut block = Vec::<Statement>::new();

        while !parser.curr_token_is(&Token::RCurly) && !parser.curr_token.reached_eof() {
            if let Some(statement) = parser.parse_statement() {
                block.push(statement);
            }

            parser.next_token();
        }

        BlockStatement { statements: block }
    }
}
