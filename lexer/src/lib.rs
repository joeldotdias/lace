pub mod token;

#[cfg(test)]
mod tests;

use token::{LiteralType, Token};

/// Iterator over the code
/// Acts as a cursor over the input
pub struct Lexer {
    /// input from the user as an array of bytes
    input: Vec<u8>,
    /// points to the current position
    position: usize,
    /// next position to be read
    read_position: usize,
    /// current character under examination as a byte
    curr_ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            curr_ch: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.curr_ch {
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
            b'/' => {
                if self.peek() == b'/' {
                    self.read_line_comment()
                } else if self.peek() == b'*' {
                    self.read_block_comment()
                } else {
                    Token::ForwardSlash
                }
            }
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
            b'|' => {
                if self.peek() == b'|' {
                    self.read_char();
                    Token::Or
                } else {
                    Token::Illegal
                }
            }
            b'&' => {
                if self.peek() == b'&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::Illegal
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
                    _ => Token::Ident { label: ident },
                };
            }
            b'0'..=b'9' => {
                return self.read_int();
            }
            b'"' => {
                return self.read_str();
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        token
    }

    fn peek(&self) -> u8 {
        if self.reached_end_of_input() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    // peeks 2 bytes ahead
    fn peek_peek(&self) -> u8 {
        if self.read_position >= self.input.len() - 1 {
            0
        } else {
            self.input[self.read_position + 1]
        }
    }

    fn read_char(&mut self) {
        if self.reached_end_of_input() {
            self.curr_ch = 0; // set ch to 0 if we reach end of input
        } else {
            self.curr_ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;

        while self.curr_ch.is_ascii_alphabetic()
            || self.curr_ch == b'_'
            || self.curr_ch.is_ascii_digit()
        {
            self.read_char();
        }

        // because we only support ASCII characters
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_int(&mut self) -> Token {
        let pos = self.position;

        while self.curr_ch.is_ascii_digit() {
            self.read_char();
        }

        Token::Literal {
            kind: LiteralType::Int,
            val: String::from_utf8_lossy(&self.input[pos..self.position]).to_string(),
        }
    }

    fn read_str(&mut self) -> Token {
        self.read_char(); // skip the opening "

        let mut estr = String::new();

        while self.curr_ch != b'"' {
            if self.curr_ch == b'\\' {
                // if a backslash is found we skip it
                // and read the next character as is
                self.read_char();
            }
            estr.push(self.curr_ch.into());
            self.read_char();
        }

        self.read_char(); // skip the closing "

        Token::Literal {
            kind: LiteralType::Str,
            val: estr,
        }
    }

    fn read_line_comment(&mut self) -> Token {
        // skip the '//' which denotes start of a comment
        self.read_char();
        self.read_char();

        let pos = self.position;

        while self.curr_ch != b'\n' && !self.reached_end_of_input() {
            self.read_char();
        }

        let cm_str = String::from_utf8_lossy(&self.input[pos..self.position]).to_string();

        self.read_char();

        Token::LineComment { content: cm_str }
    }

    fn read_block_comment(&mut self) -> Token {
        // skip the '/*' which denotes start of a comment
        self.read_char();
        self.read_char();

        let pos = self.position;
        let mut terminated = true;

        while self.peek() != b'*' || self.peek_peek() != b'/' {
            if self.reached_end_of_input() {
                terminated = false;
                break;
            }

            self.read_char();
        }

        let cm_str = String::from_utf8_lossy(&self.input[pos..self.position + 1]).to_string();

        self.read_char();

        Token::BlockComment {
            content: cm_str,
            terminated,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn reached_end_of_input(&self) -> bool {
        self.read_position >= self.input.len()
    }
}
