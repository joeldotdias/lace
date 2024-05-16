use lace_lexer::{token::Token, Lexer};

use crate::{
    ast::{
        nodes::{IdentNode, PrimitiveNode},
        statement::{LetStatement, Statement},
        Expression,
    },
    Parser,
};

pub fn validate_parser(input: &str, expected_statemets: Vec<Statement>) {
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(check_parser_errors(&parser), 0);

    assert_eq!(program.statements.len(), expected_statemets.len());

    for (i, expected) in expected_statemets.iter().enumerate() {
        println!("{} | {} | {} ", i, expected, program.statements[i]);
        assert_eq!(program.statements[i], *expected);
    }
}

fn check_parser_errors(parser: &Parser) -> usize {
    if !parser.errors.is_empty() {
        println!("Parser has {} errors:", parser.errors.len());
        parser.log_errors();
    }

    parser.errors.len()
}

#[test]
fn will_you_parse_let() {
    println!("Hello");
    let input = r#"
        let x = 5;
        let y = 10;
        let flag = false;
        let foobar = y;
    "#;

    let expected_statemets = vec![
        Statement::Let(LetStatement {
            name: IdentNode {
                token: Token::Ident("x".to_string()),
                val: "x".to_string(),
            },
            val: Expression::Primitive(PrimitiveNode::IntegerLiteral(5)),
        }),
        Statement::Let(LetStatement {
            name: IdentNode {
                token: Token::Ident("y".to_string()),
                val: "y".to_string(),
            },
            val: Expression::Primitive(PrimitiveNode::IntegerLiteral(10)),
        }),
        Statement::Let(LetStatement {
            name: IdentNode {
                token: Token::Ident("flag".to_string()),
                val: "flag".to_string(),
            },
            val: Expression::Primitive(PrimitiveNode::BooleanLiteral(false)),
        }),
        Statement::Let(LetStatement {
            name: IdentNode {
                token: Token::Ident("foobar".to_string()),
                val: "foobar".to_string(),
            },
            val: Expression::Identifier(IdentNode {
                token: Token::Ident("y".to_string()),
                val: "y".to_string(),
            }),
        }),
    ];

    validate_parser(input, expected_statemets);
}

#[test]
fn will_you_oopsie_let() {
    println!("");
    let input = r"
        let x 5;
        let = 10;
        let foobar == y;
    ";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);
    parser.parse_program();

    parser.log_errors();
    assert_ne!(parser.errors.len(), 0);
}
