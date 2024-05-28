use std::{fmt::Display, path::PathBuf};

use lace_lexer::token::{dummy_token, kind::TokenKind};

use crate::{
    ast::{nodes::IdentNode, Expression},
    Parser,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Assignment(LetStatement),
    Return(ReturnStatement),
    Expr(Expression),
    Source(SourceStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment(statement) => write!(f, "{}", statement),
            Statement::Return(statement) => write!(f, "{}", statement),
            Statement::Expr(expression) => write!(f, "{}", expression),
            Statement::Source(source) => write!(f, "{}", source),
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
        write!(f, "Let => {{ {} | Assigned | {} }}", self.name, self.val)
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
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut block = String::new();
        self.statements.iter().for_each(|statement| {
            block.push_str(format!("{}\n", statement).as_str());
        });
        write!(f, "{}", block)
    }
}

impl BlockStatement {
    pub fn parse(parser: &mut Parser) -> Self {
        parser.next_token();

        let mut block = Vec::<Statement>::new();

        while !parser.curr_token_is(&dummy_token(TokenKind::RCurly))
            && !parser.curr_token.reached_eof()
        {
            if let Some(statement) = parser.parse_statement() {
                block.push(statement);
            }

            parser.next_token();
        }

        BlockStatement { statements: block }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SourceStatement {
    pub path: PathBuf,
}

impl Display for SourceStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source {}", self.path.to_str().unwrap())
    }
}
