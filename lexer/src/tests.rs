use std::fs;

use super::{Lexer, LiteralKind, Token};

fn validate_tokens(input: &str, tokens: Vec<Token>) {
    let mut lexer = Lexer::new(input.into());

    for token in tokens {
        let next_token = lexer.next_token();
        println!("expected: {:?}, received {:?}", token, next_token);
        assert_eq!(token, next_token);
    }
}

#[test]
fn will_you_lex() {
    let input = "=+(.){},;%";

    let tokens = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::Dot,
        Token::RParen,
        Token::LCurly,
        Token::RCurly,
        Token::Comma,
        Token::Semicolon,
        Token::Modulo,
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
        Token::Let,
        Token::Ident {
            label: "five".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "ten".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "add".into(),
        },
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident { label: "x".into() },
        Token::Comma,
        Token::Ident { label: "y".into() },
        Token::RParen,
        Token::LCurly,
        Token::Ident { label: "x".into() },
        Token::Plus,
        Token::Ident { label: "y".into() },
        Token::RCurly,
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "result".into(),
        },
        Token::Assign,
        Token::Ident {
            label: "add".into(),
        },
        Token::LParen,
        Token::Ident {
            label: "five".into(),
        },
        Token::Comma,
        Token::Ident {
            label: "ten".into(),
        },
        Token::RParen,
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "greet".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from("Hi, my age is 10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "flag".into(),
        },
        Token::Assign,
        Token::True,
        Token::Semicolon,
        Token::Let,
        Token::Ident { label: "ch".into() },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Char { terminated: true },
            val: String::from("b"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident { label: "fl".into() },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Float,
            val: String::from("40.627"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident { label: "ln".into() },
        Token::Assign,
        Token::Ident {
            label: "greet".into(),
        },
        Token::Dot,
        Token::Ident {
            label: "len".into(),
        },
        Token::LParen,
        Token::RParen,
        Token::Semicolon,
        Token::Eof,
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
        Token::Let,
        Token::Ident {
            label: "five".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "ten".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "add".into(),
        },
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident { label: "x".into() },
        Token::Comma,
        Token::Ident { label: "y".into() },
        Token::RParen,
        Token::LCurly,
        Token::Ident { label: "x".into() },
        Token::Plus,
        Token::Ident { label: "y".into() },
        Token::Semicolon,
        Token::RCurly,
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "result".into(),
        },
        Token::Assign,
        Token::Ident {
            label: "add".into(),
        },
        Token::LParen,
        Token::Ident {
            label: "five".into(),
        },
        Token::Comma,
        Token::Ident {
            label: "ten".into(),
        },
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::ForwardSlash,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::Asterisk,
        Token::Semicolon,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::LessThan,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::GreaterThan,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::LessThan,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::RParen,
        Token::LCurly,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RCurly,
        Token::Else,
        Token::LCurly,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RCurly,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("5"),
        },
        Token::LessThanEqual,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::Equal,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("10"),
        },
        Token::NotEqual,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("9"),
        },
        Token::Semicolon,
        Token::Eof,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn will_you_lex_from_a_file() {
    let contents = fs::read_to_string("../examples/basic.lace").unwrap();
    let input = contents.as_str();

    let tokens = vec![
        Token::Function,
        Token::Ident {
            label: "main".into(),
        },
        Token::LParen,
        Token::RParen,
        Token::LCurly,
        Token::Let,
        Token::Ident {
            label: "num1".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("69"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "num2".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: String::from("420"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "bigger_of_the_2".into(),
        },
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident { label: "x".into() },
        Token::Comma,
        Token::Ident { label: "y".into() },
        Token::RParen,
        Token::LCurly,
        Token::If,
        Token::Ident { label: "x".into() },
        Token::GreaterThan,
        Token::Ident { label: "y".into() },
        Token::LCurly,
        Token::Ident { label: "x".into() },
        Token::RCurly,
        Token::Else,
        Token::LCurly,
        Token::Ident { label: "y".into() },
        Token::RCurly,
        Token::RCurly,
        Token::Semicolon,
        Token::RCurly,
        Token::Eof,
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
        Token::Let,
        Token::Ident {
            label: "msg".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from(r#"He said, "Lemons taste good.""#),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            label: "other".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Str { terminated: true },
            val: String::from(r#"Escape the \escape"#),
        },
        Token::Semicolon,
        Token::Eof,
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
        Token::LineComment {
            content: " Hello".into(),
        },
        Token::LineComment {
            content: "What is up".into(),
        },
        Token::Let,
        Token::Ident {
            label: "count".into(),
        },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Int,
            val: "0".into(),
        },
        Token::Semicolon,
        Token::LineComment {
            content: " This is a counter".into(),
        },
        Token::Ident {
            label: "count".into(),
        },
        Token::Assign,
        Token::Ident {
            label: "count".into(),
        },
        Token::Plus,
        Token::Literal {
            kind: LiteralKind::Int,
            val: "1".into(),
        },
        Token::Semicolon,
        Token::BlockComment {
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

    let tokens = vec![Token::BlockComment {
        content: "This is an\nunterminated\nblock comment".into(),
        terminated: false,
    }];

    validate_tokens(input, tokens)
}

#[test]
fn detect_unterminated_string() {
    let input = r#"let s = "unterm;"#;

    let tokens = vec![
        Token::Let,
        Token::Ident { label: "s".into() },
        Token::Assign,
        Token::Literal {
            kind: LiteralKind::Str { terminated: false },
            val: "unterm".into(),
        },
        Token::Semicolon,
    ];

    validate_tokens(input, tokens)
}

#[test]
fn detect_illegal() {
    let input = "]+!Γ¡É≡ƒÜª";

    let tokens = vec![
        Token::RBracket,
        Token::Plus,
        Token::Bang,
        Token::Illegal,
        Token::Illegal,
    ];

    validate_tokens(input, tokens)
}
