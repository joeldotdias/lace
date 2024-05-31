use std::path::PathBuf;

use lace_lexer::{
    token::{dummy_token, kind::TokenKind},
    Lexer,
};

use crate::{
    ast::{
        nodes::{IdentNode, PrimitiveNode},
        statement::{LetStatement, SourceStatement, Statement},
        ExpressionKind,
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
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident { label: "x".into() }),
                label: "x".to_string(),
            },
            val: ExpressionKind::Primitive(PrimitiveNode::IntegerLiteral(5)),
        }),
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident { label: "y".into() }),
                label: "y".to_string(),
            },
            val: ExpressionKind::Primitive(PrimitiveNode::IntegerLiteral(10)),
        }),
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident {
                    label: "flag".to_string(),
                }),
                label: "flag".to_string(),
            },
            val: ExpressionKind::Primitive(PrimitiveNode::BooleanLiteral(false)),
        }),
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident {
                    label: "foobar".to_string(),
                }),
                label: "foobar".to_string(),
            },
            val: ExpressionKind::Identifier(IdentNode {
                token: dummy_token(TokenKind::Ident {
                    label: "y".to_string(),
                }),
                label: "y".to_string(),
            }),
        }),
    ];

    validate_parser(input, expected_statemets);
}

// #[test]
// fn will_you_oopsie_let() {
//     println!("");
//     let input = r"
//         let x 5;
//         let = 10;
//         let foobar == y;
//     ";
//     let lexer = Lexer::new(input.into());
//     let mut parser = Parser::new(lexer);
//     parser.parse_program();
//
//     parser.log_errors();
//     println!("len is: {}", parser.errors.len());
//     assert_ne!(parser.errors.len(), 0);
// }

#[test]
fn will_you_parse_source() {
    let input = r#"
        let x = 5;
        source "path/to/source";
        let y = 10;
    "#;
    let expected_statemets = vec![
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident { label: "x".into() }),
                label: "x".to_string(),
            },
            val: ExpressionKind::Primitive(PrimitiveNode::IntegerLiteral(5)),
        }),
        Statement::Source(SourceStatement {
            path: PathBuf::from("path/to/source"),
        }),
        Statement::Assignment(LetStatement {
            name: IdentNode {
                token: dummy_token(TokenKind::Ident { label: "y".into() }),
                label: "y".to_string(),
            },
            val: ExpressionKind::Primitive(PrimitiveNode::IntegerLiteral(10)),
        }),
    ];

    validate_parser(input, expected_statemets)
}
