use std::fs;

use crate::{
    token::kind::{LiteralKind, TokenKind},
    Lexer,
};

fn validate_tokens(input: &str, tokens: Vec<TokenKind>) {
    let mut lexer = Lexer::new(input.into());

    for expected_token_kind in tokens {
        let received_token = lexer.next_token();
        println!(
            "expected: {}, received {}",
            expected_token_kind, received_token
        );
        assert_eq!(expected_token_kind, received_token.kind);
    }
}

#[test]
fn will_you_lex() {
    let input = "=+(.){},;%";

    let tokens = vec![
        TokenKind::Assign,
        TokenKind::Plus,
        TokenKind::LParen,
        TokenKind::Dot,
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::RCurly,
        TokenKind::Comma,
        TokenKind::Semicolon,
        TokenKind::Modulo,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_lex_some_code() {
    let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y
        };

        let result = add(five, ten);
        let greet = "Hi, my age is 10";
        let flag = true;
        let ch = 'b';
        let fl = 40.627;
        let ln = greet.len();
        "#;

    let tokens = vec![
        TokenKind::Let,
        TokenKind::Ident {
            label: "five".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "ten".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "add".into(),
        },
        TokenKind::Assign,
        TokenKind::Function,
        TokenKind::LParen,
        TokenKind::Ident { label: "x".into() },
        TokenKind::Comma,
        TokenKind::Ident { label: "y".into() },
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::Ident { label: "x".into() },
        TokenKind::Plus,
        TokenKind::Ident { label: "y".into() },
        TokenKind::RCurly,
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "result".into(),
        },
        TokenKind::Assign,
        TokenKind::Ident {
            label: "add".into(),
        },
        TokenKind::LParen,
        TokenKind::Ident {
            label: "five".into(),
        },
        TokenKind::Comma,
        TokenKind::Ident {
            label: "ten".into(),
        },
        TokenKind::RParen,
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "greet".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from("Hi, my age is 10"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "flag".into(),
        },
        TokenKind::Assign,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident { label: "ch".into() },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: true },
            val: String::from("b"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident { label: "fl".into() },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Float,
            val: String::from("40.627"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident { label: "ln".into() },
        TokenKind::Assign,
        TokenKind::Ident {
            label: "greet".into(),
        },
        TokenKind::Dot,
        TokenKind::Ident {
            label: "len".into(),
        },
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_lex_more_code() {
    let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        !-/5*;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        5 <= 10;
        10 == 10;
        10 != 9;
        "#;

    let tokens = vec![
        TokenKind::Let,
        TokenKind::Ident {
            label: "five".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "ten".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "add".into(),
        },
        TokenKind::Assign,
        TokenKind::Function,
        TokenKind::LParen,
        TokenKind::Ident { label: "x".into() },
        TokenKind::Comma,
        TokenKind::Ident { label: "y".into() },
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::Ident { label: "x".into() },
        TokenKind::Plus,
        TokenKind::Ident { label: "y".into() },
        TokenKind::Semicolon,
        TokenKind::RCurly,
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "result".into(),
        },
        TokenKind::Assign,
        TokenKind::Ident {
            label: "add".into(),
        },
        TokenKind::LParen,
        TokenKind::Ident {
            label: "five".into(),
        },
        TokenKind::Comma,
        TokenKind::Ident {
            label: "ten".into(),
        },
        TokenKind::RParen,
        TokenKind::Semicolon,
        TokenKind::Bang,
        TokenKind::Minus,
        TokenKind::ForwardSlash,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::Asterisk,
        TokenKind::Semicolon,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::LessThan,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::GreaterThan,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::Semicolon,
        TokenKind::If,
        TokenKind::LParen,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::LessThan,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::Return,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::RCurly,
        TokenKind::Else,
        TokenKind::LCurly,
        TokenKind::Return,
        TokenKind::False,
        TokenKind::Semicolon,
        TokenKind::RCurly,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        TokenKind::LessThanEqual,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::Semicolon,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::Equal,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::Semicolon,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        TokenKind::NotEqual,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("9"),
        },
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_lex_from_a_file() {
    let contents = fs::read_to_string("../docs/examples/basic.lace").unwrap();
    let input = contents.as_str();

    let tokens = vec![
        TokenKind::Function,
        TokenKind::Ident {
            label: "main".into(),
        },
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::Let,
        TokenKind::Ident {
            label: "num1".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("69"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "num2".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: String::from("420"),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "bigger_of_the_2".into(),
        },
        TokenKind::Assign,
        TokenKind::Function,
        TokenKind::LParen,
        TokenKind::Ident { label: "x".into() },
        TokenKind::Comma,
        TokenKind::Ident { label: "y".into() },
        TokenKind::RParen,
        TokenKind::LCurly,
        TokenKind::If,
        TokenKind::Ident { label: "x".into() },
        TokenKind::GreaterThan,
        TokenKind::Ident { label: "y".into() },
        TokenKind::LCurly,
        TokenKind::Ident { label: "x".into() },
        TokenKind::RCurly,
        TokenKind::Else,
        TokenKind::LCurly,
        TokenKind::Ident { label: "y".into() },
        TokenKind::RCurly,
        TokenKind::RCurly,
        TokenKind::Semicolon,
        TokenKind::RCurly,
        TokenKind::Eof,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_escape() {
    // This test checks for two things
    // Will a character after a backslash be properly excaped
    // Will a backslash after a backslash also be excaped

    let input = r#"
        let msg = "He said, \"Lemons taste good.\"";
        let other = "Escape the \\escape";
    "#;

    let tokens = vec![
        TokenKind::Let,
        TokenKind::Ident {
            label: "msg".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from(r#"He said, "Lemons taste good.""#),
        },
        TokenKind::Semicolon,
        TokenKind::Let,
        TokenKind::Ident {
            label: "other".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from(r#"Escape the \escape"#),
        },
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_lex_a_comment() {
    let input = r#"
        // Hello
        //What is up
        let count = 0; // This is a counter
        count = count + 1;
        /* This is a
block comment */
    "#;

    let tokens = vec![
        TokenKind::LineComment {
            content: " Hello".into(),
        },
        TokenKind::LineComment {
            content: "What is up".into(),
        },
        TokenKind::Let,
        TokenKind::Ident {
            label: "count".into(),
        },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: "0".into(),
        },
        TokenKind::Semicolon,
        TokenKind::LineComment {
            content: " This is a counter".into(),
        },
        TokenKind::Ident {
            label: "count".into(),
        },
        TokenKind::Assign,
        TokenKind::Ident {
            label: "count".into(),
        },
        TokenKind::Plus,
        TokenKind::Literal {
            kind: LiteralKind::Int,
            val: "1".into(),
        },
        TokenKind::Semicolon,
        TokenKind::BlockComment {
            content: " This is a\nblock comment ".into(),
            terminated: true,
        },
    ];

    validate_tokens(input, tokens)
}

#[test]
fn detect_unterminated_comment() {
    let input = r#"
        /*This is an
unterminated
block comment"#;

    let tokens = vec![TokenKind::BlockComment {
        content: "This is an\nunterminated\nblock comment".into(),
        terminated: false,
    }];

    validate_tokens(input, tokens)
}

#[test]
fn detect_unterminated_string() {
    let input = r#"let s = "unterm;"#;

    let tokens = vec![
        TokenKind::Let,
        TokenKind::Ident { label: "s".into() },
        TokenKind::Assign,
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: false },
            val: "unterm".into(),
        },
        TokenKind::Semicolon,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn detect_illegal() {
    let input = "]+!Γ¡É≡ƒÜª";

    let tokens = vec![
        TokenKind::RBracket,
        TokenKind::Plus,
        TokenKind::Bang,
        TokenKind::Illegal,
        TokenKind::Illegal,
    ];

    validate_tokens(input, tokens)
}
