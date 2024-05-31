use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Span {
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.end_col < 1 {
            return write!(f, "EOF");
        }

        let range = if self.start_col == self.end_col - 1 {
            format!("{}", self.start_col + 1)
        } else {
            format!("{} to {}", self.start_col + 1, self.end_col)
        };

        let lines = if self.start_line == self.end_line {
            format!("line {}", self.start_line)
        } else {
            format!("lines {} to {}", self.start_line, self.end_line)
        };

        write!(f, "{range} on {lines}")
    }
}

pub fn dummy_span() -> Span {
    Span {
        start_line: 0,
        end_line: 0,
        start_col: 0,
        end_col: 0,
    }
}
