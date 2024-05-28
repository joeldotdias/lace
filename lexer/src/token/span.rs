use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Span {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = if self.start_col == self.end_col - 1 {
            format!("{}", self.start_col)
        } else {
            format!("{} to {}", self.start_col, self.end_col - 1)
        };

        let lines = if self.start_row == self.end_row {
            format!("line {}", self.start_row)
        } else {
            format!("lines {} to {}", self.start_row, self.end_row)
        };

        write!(f, "{} on {}", range, lines)
    }
}

pub fn dummy_span() -> Span {
    Span {
        start_row: 0,
        end_row: 0,
        start_col: 0,
        end_col: 0,
    }
}
