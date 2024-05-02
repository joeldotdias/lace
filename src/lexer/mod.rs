pub mod token;

use crate::lexer::token::{Token, LiteralType};

/// Iterator over the code
pub struct Lexer {
    /// input from the user as an array of bytes
    input: Vec<u8>,
    /// points to the current position
    position: usize,
    /// next position to be read
    read_position: usize,
    /// current character under examination as a byte
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => Token::LCurly,
            b'}' => Token::RCurly,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::ForwardSlash,
            b'%' => Token::Modulo,
            b'<' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();

                return match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "true" => Token::True,
                    "false" => Token::False,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                };
            }
            // b'0'..=b'9' => return Token::Int(self.read_int()),
            b'0'..=b'9' => return Token::Literal { kind: LiteralType::Int, val: self.read_int() },
            b'"' => return Token::Literal { kind: LiteralType::Str, val: self.read_str() },
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // set ch to 0 if we reach end of input --> 0 is ascii for null
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.ch.is_ascii_digit() {
            self.read_char();
        }

        // because we only support ASCII characters
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_str(&mut self) -> String {
        self.read_char(); // skip the opening "
        let pos = self.position;

        while self.ch != b'"' {
            self.read_char();
        }

        self.read_char(); // skip the closing "

        String::from_utf8_lossy(&self.input[pos..self.position - 1]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::{Lexer, Token, LiteralType};

    fn validate_tokens(lexer: &mut Lexer, tokens: Vec<Token>) {
        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn will_you_lex() {
        let input = "=+(){},;%";
        let mut lexer = Lexer::new(input.into());

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

        validate_tokens(&mut lexer, tokens)
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

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
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
            Token::Literal { kind: LiteralType::Str, val: String::from("Hi, my age is 10") },
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("flag")),
            Token::Assign,
            Token::True,
            Token::Semicolon,
            Token::Eof,
        ];

        validate_tokens(&mut lexer, tokens)
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

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
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
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::Semicolon,
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::LessThan,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
            Token::GreaterThan,
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::LessThan,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
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
            Token::Literal { kind: LiteralType::Int, val: String::from("5") },
            Token::LessThanEqual,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
            Token::Semicolon,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
            Token::Equal,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
            Token::Semicolon,
            Token::Literal { kind: LiteralType::Int, val: String::from("10") },
            Token::NotEqual,
            Token::Literal { kind: LiteralType::Int, val: String::from("9") },
            Token::Semicolon,
            Token::Eof,
        ];

        validate_tokens(&mut lexer, tokens)
    }

    #[test]
    fn will_you_lex_from_a_file() {
        let input = fs::read_to_string("examples/basic.lace").unwrap();
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Function,
            Token::Ident("main".into()),
            Token::LParen,
            Token::RParen,
            Token::LCurly,
            Token::Let,
            Token::Ident("num1".into()),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("69") },
            Token::Semicolon,
            Token::Let,
            Token::Ident("num2".into()),
            Token::Assign,
            Token::Literal { kind: LiteralType::Int, val: String::from("420") },
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

        validate_tokens(&mut lexer, tokens)
    }

    #[test]
    fn detect_illegal() {
        let input = "]+!⭐🚦";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::RBracket,
            Token::Plus,
            Token::Bang,
            Token::Illegal,
            Token::Illegal,
        ];

        validate_tokens(&mut lexer, tokens)
    }
}
