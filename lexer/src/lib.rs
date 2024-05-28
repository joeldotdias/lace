pub mod token;

#[cfg(test)]
mod tests;

use token::{
    kind::{LiteralKind, TokenKind},
    span::Span,
    Token,
};

/// Iterator over the code
/// Acts as a cursor over the input
pub struct Lexer {
    /// input from the user as a vector of bytes
    input: Vec<u8>,
    /// points to the current position
    position: usize,
    /// next position to be read
    read_position: usize,
    /// current character under examination as a byte
    curr_ch: u8,
    /// position of each line break
    line_breaks: Vec<usize>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut line_breaks = vec![0];
        line_breaks.extend(
            input
                .chars()
                .enumerate()
                .filter_map(|(ln, ch)| (ch == '\n').then_some(ln + 1)),
        );
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            curr_ch: 0,
            line_breaks,
        };

        lexer.advance_byte();

        lexer
    }

    pub fn make_span(&self, start_pos: usize) -> Span {
        let end_pos = self.position;

        let start_line = match self
            .line_breaks
            .iter()
            .enumerate()
            .find_map(|(ln, &ch)| (ch > start_pos).then_some(ln))
        {
            Some(n) => n,
            None => self.line_breaks.len(),
        };

        let end_line = match self
            .line_breaks
            .iter()
            .enumerate()
            .find_map(|(ln, &ch)| (ch + 1 > end_pos).then_some(ln))
        {
            Some(n) => n,
            None => self.line_breaks.len(),
        };

        Span {
            start_line,
            end_line,
            start_col: start_pos - self.line_breaks[start_line - 1],
            end_col: end_pos - self.line_breaks[end_line - 1],
        }
    }

    pub fn token_kind(&mut self) -> TokenKind {
        let token = match self.curr_ch {
            b'{' => TokenKind::LCurly,
            b'}' => TokenKind::RCurly,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b'.' => TokenKind::Dot,
            b',' => TokenKind::Comma,
            b':' => TokenKind::Colon,
            b';' => TokenKind::Semicolon,
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'!' => {
                if self.peek() == b'=' {
                    self.advance_byte();
                    TokenKind::NotEqual
                } else {
                    TokenKind::Bang
                }
            }
            b'*' => TokenKind::Asterisk,
            b'/' => {
                if self.peek() == b'/' {
                    self.read_line_comment()
                } else if self.peek() == b'*' {
                    self.read_block_comment()
                } else {
                    TokenKind::ForwardSlash
                }
            }
            b'%' => TokenKind::Modulo,
            b'<' => {
                if self.peek() == b'=' {
                    self.advance_byte();
                    TokenKind::LessThanEqual
                } else {
                    TokenKind::LessThan
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.advance_byte();
                    TokenKind::GreaterThanEqual
                } else {
                    TokenKind::GreaterThan
                }
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.advance_byte();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            b'|' => {
                if self.peek() == b'|' {
                    self.advance_byte();
                    TokenKind::Or
                } else {
                    TokenKind::Illegal
                }
            }
            b'&' => {
                if self.peek() == b'&' {
                    self.advance_byte();
                    TokenKind::And
                } else {
                    TokenKind::Illegal
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let label = self.read_ident();

                return match TokenKind::try_keyword(&label) {
                    Some(keyword) => keyword,
                    None => TokenKind::Ident { label },
                };
            }
            b'0'..=b'9' => {
                return self.read_int();
            }
            b'\'' => {
                return self.read_char();
            }
            b'"' => {
                return self.read_str();
            }
            0 => TokenKind::Eof,
            _ => TokenKind::Illegal,
        };

        self.advance_byte();

        token
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.position;

        let kind = self.token_kind();

        Token {
            kind,
            span: self.make_span(start),
        }
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

    fn advance_byte(&mut self) {
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
            self.advance_byte();
        }

        // because we only support ASCII characters
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_int(&mut self) -> TokenKind {
        let pos = self.position;
        let mut dot = false;

        while self.curr_ch.is_ascii_digit() || self.curr_ch == b'.' {
            if self.curr_ch == b'.' {
                dot = true;
            }
            self.advance_byte();
        }

        let kind = if dot {
            LiteralKind::Float
        } else {
            LiteralKind::Int
        };

        TokenKind::Literal {
            kind,
            val: String::from_utf8_lossy(&self.input[pos..self.position]).to_string(),
        }
    }

    fn read_str(&mut self) -> TokenKind {
        self.advance_byte(); // skip the opening "

        let mut estr = String::new();
        let mut terminated = true;

        while self.curr_ch != b'"' {
            if self.curr_ch == b'\\' {
                // if a backslash is found we skip it
                // and read the next character as is
                self.advance_byte();
            }

            estr.push(self.curr_ch.into());
            self.advance_byte();

            if self.reached_end_of_input() {
                terminated = false;
                break;
            }
        }

        if terminated {
            self.advance_byte(); // skip the closing "
        }

        TokenKind::Literal {
            kind: LiteralKind::Str { terminated },
            val: estr,
        }
    }

    fn read_char(&mut self) -> TokenKind {
        self.advance_byte();

        let mut val = String::new();
        val.push(self.curr_ch.into());
        self.advance_byte();

        let terminated = self.curr_ch == b'\'';
        if terminated {
            self.advance_byte();
        }

        TokenKind::Literal {
            kind: LiteralKind::Char { terminated },
            val,
        }
    }

    fn read_line_comment(&mut self) -> TokenKind {
        // skip the '//' which denotes start of a comment
        self.advance_byte();
        self.advance_byte();

        let pos = self.position;

        while self.curr_ch != b'\n' && !self.reached_end_of_input() {
            self.advance_byte();
        }

        let cm_str = String::from_utf8_lossy(&self.input[pos..self.position]).to_string();

        self.advance_byte();

        TokenKind::LineComment { content: cm_str }
    }

    fn read_block_comment(&mut self) -> TokenKind {
        // skip the '/*' which denotes start of a comment
        self.advance_byte();
        self.advance_byte();

        let pos = self.position;
        let mut terminated = true;

        while self.peek() != b'*' || self.peek_peek() != b'/' {
            if self.reached_end_of_input() {
                terminated = false;
                break;
            }

            self.advance_byte();
        }

        let cm_str = String::from_utf8_lossy(&self.input[pos..self.position + 1]).to_string();

        self.advance_byte();

        TokenKind::BlockComment {
            content: cm_str,
            terminated,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_ch.is_ascii_whitespace() {
            self.advance_byte();
        }
    }

    fn reached_end_of_input(&self) -> bool {
        self.read_position >= self.input.len()
    }
}
