use std::fs;

use super::{Lexer, LiteralType, Token};

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
    let input = "=+(){},;%";

    let tokens = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
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
        "#;

    let tokens = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LCurly,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::RCurly,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::LParen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::RParen,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("greet")),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Str,
            val: String::from("Hi, my age is 10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("flag")),
        Token::Assign,
        Token::True,
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
        !-/*5;
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
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LCurly,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RCurly,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::LParen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::ForwardSlash,
        Token::Asterisk,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::LessThan,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::GreaterThan,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::LessThan,
        Token::Literal {
            kind: LiteralType::Int,
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
            kind: LiteralType::Int,
            val: String::from("5"),
        },
        Token::LessThanEqual,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::Equal,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::Semicolon,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("10"),
        },
        Token::NotEqual,
        Token::Literal {
            kind: LiteralType::Int,
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
        Token::Ident("main".into()),
        Token::LParen,
        Token::RParen,
        Token::LCurly,
        Token::Let,
        Token::Ident("num1".into()),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("69"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident("num2".into()),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Int,
            val: String::from("420"),
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident("bigger_of_the_2".into()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".into()),
        Token::Comma,
        Token::Ident("y".into()),
        Token::RParen,
        Token::LCurly,
        Token::If,
        Token::Ident("x".into()),
        Token::GreaterThan,
        Token::Ident("y".into()),
        Token::LCurly,
        Token::Ident("x".into()),
        Token::RCurly,
        Token::Else,
        Token::LCurly,
        Token::Ident("y".into()),
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
        Token::Ident("msg".into()),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Str,
            val: String::from(r#"He said, "Lemons taste good.""#)
        },
        Token::Semicolon,
        Token::Let,
        Token::Ident("other".into()),
        Token::Assign,
        Token::Literal {
            kind: LiteralType::Str,
            val: String::from(r#"Escape the \escape"#)
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
    "#;

    let tokens = vec![
        Token::LineComment(" Hello".into()),
        Token::LineComment("What is up".into()),
        Token::Let,
        Token::Ident("count".into()),
        Token::Assign,
        Token::Literal { kind: LiteralType::Int, val: "0".into() },
        Token::Semicolon,
        Token::LineComment(" This is a counter".into()),
        Token::Ident("count".into()),
        Token::Assign,
        Token::Ident("count".into()),
        Token::Plus,
        Token::Literal { kind: LiteralType::Int, val: "1".into() },
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
