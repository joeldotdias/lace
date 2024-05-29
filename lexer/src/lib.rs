pub mod token;

#[cfg(test)]
mod tests;

use token::{
    kind::{LiteralKind, TokenKind},
    span::Span,
    Token, EOF_CHAR,
};

/// Iterator over the code
/// Acts as a cursor over the input
pub struct Lexer {
    /// input from the user as a vector of characters
    input: Vec<char>,
    /// points to the current position
    position: usize,
    /// next position to be read
    read_position: usize,
    /// current character under examination as a byte
    curr_ch: char,
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
            // input: input.into_bytes(),
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            curr_ch: EOF_CHAR,
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
            '{' => TokenKind::LCurly,
            '}' => TokenKind::RCurly,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '!' => {
                if self.peek() == '=' {
                    self.advance_byte();
                    TokenKind::NotEqual
                } else {
                    TokenKind::Bang
                }
            }
            '*' => TokenKind::Asterisk,
            '/' => {
                if self.peek() == '/' {
                    self.read_line_comment()
                } else if self.peek() == '*' {
                    self.read_block_comment()
                } else {
                    TokenKind::ForwardSlash
                }
            }
            '%' => TokenKind::Modulo,
            '<' => {
                if self.peek() == '=' {
                    self.advance_byte();
                    TokenKind::LessThanEqual
                } else {
                    TokenKind::LessThan
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance_byte();
                    TokenKind::GreaterThanEqual
                } else {
                    TokenKind::GreaterThan
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance_byte();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance_byte();
                    TokenKind::Or
                } else {
                    TokenKind::Illegal { ch: self.curr_ch }
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.advance_byte();
                    TokenKind::And
                } else {
                    TokenKind::Illegal { ch: self.curr_ch }
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let label = self.read_ident();

                return match TokenKind::try_keyword(&label) {
                    Some(keyword) => keyword,
                    None => TokenKind::Ident { label },
                };
            }
            '0'..='9' => {
                return self.read_int();
            }
            '\'' => {
                return self.read_char();
            }
            '"' => {
                return self.read_str();
            }
            EOF_CHAR => TokenKind::Eof,
            _ => TokenKind::Illegal { ch: self.curr_ch },
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

    fn peek(&self) -> char {
        if self.reached_end_of_input() {
            EOF_CHAR
        } else {
            self.input[self.read_position]
        }
    }

    // peeks 2 bytes ahead
    fn peek_peek(&self) -> char {
        if self.read_position >= self.input.len() - 1 {
            EOF_CHAR
        } else {
            self.input[self.read_position + 1]
        }
    }

    fn advance_byte(&mut self) {
        if self.reached_end_of_input() {
            self.curr_ch = EOF_CHAR;
        } else {
            self.curr_ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;

        while self.curr_ch.is_ascii_alphabetic()
            || self.curr_ch == '_'
            || self.curr_ch.is_ascii_digit()
        {
            self.advance_byte();
        }

        self.input[pos..self.position].iter().collect::<String>()
    }

    fn read_int(&mut self) -> TokenKind {
        let pos = self.position;
        let mut dot = false;

        while self.curr_ch.is_ascii_digit() || self.curr_ch == '.' {
            if self.curr_ch == '.' {
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
            val: self.input[pos..self.position].iter().collect::<String>(),
        }
    }

    fn read_str(&mut self) -> TokenKind {
        self.advance_byte(); // skip the opening "

        let mut estr = String::new();
        let mut terminated = true;

        while self.curr_ch != '"' {
            if self.curr_ch == '\\' {
                // if a backslash is found we skip it
                // and read the next character as is
                self.advance_byte();
            }

            estr.push(self.curr_ch);
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
        val.push(self.curr_ch);
        self.advance_byte();

        let terminated = self.curr_ch == '\'';
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

        while self.curr_ch != '\n' && !self.reached_end_of_input() {
            self.advance_byte();
        }

        let cm_str = self.input[pos..self.position].iter().collect::<String>();

        self.advance_byte();

        TokenKind::LineComment { content: cm_str }
    }

    fn read_block_comment(&mut self) -> TokenKind {
        // skip the '/*' which denotes start of a comment
        self.advance_byte();
        self.advance_byte();

        let pos = self.position;
        let mut terminated = true;

        while self.peek() != '*' || self.peek_peek() != '/' {
            if self.reached_end_of_input() {
                terminated = false;
                break;
            }

            self.advance_byte();
        }

        let cm_str = self.input[pos..self.position + 1]
            .iter()
            .collect::<String>();

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
