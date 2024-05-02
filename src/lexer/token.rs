use std::fmt::Display;

/// Reprsents the parsed token type.
/// In case of a string or integer literal,
/// we store the datatype and value
#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Literal { kind: LiteralType, val: String },

    // Operators
    Assign, // =
    Bang, // !
    Plus, // +
    Minus, // -
    ForwardSlash, // /
    Modulo, // %
    Asterisk, // *
    Equal, // ==
    NotEqual, // !=
    LessThan, // <
    GreaterThan, // >
    LessThanEqual, // <=
    GreaterThanEqual, // >=

    // Delimiters
    Comma, // ,
    Semicolon, // ;
    LParen, // (
    RParen, // )
    LCurly, // {
    RCurly, // }
    LBracket, // [
    RBracket, // ]

    // Keywords
    Let,
    Function,
    Return,
    If,
    Else,
    True,
    False,

    Eof,
    Illegal,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Literal { kind, val } => write!(f, "{:?} Literal({})", kind, val),
            Token::Assign => write!(f, "Assign"),
            Token::Bang => write!(f, "Bang"),
            Token::Minus => write!(f, "Minus"),
            Token::Plus => write!(f, "Plus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::ForwardSlash => write!(f, "ForwardSlash"),
            Token::Modulo => write!(f, "Modulo"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::LessThanEqual => write!(f, "LessThanEqual"),
            Token::GreaterThanEqual => write!(f, "GreaterThanEqual"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::LParen => write!(f, "Lparen"),
            Token::RParen => write!(f, "Rparen"),
            Token::LCurly => write!(f, "LSquirly"),
            Token::RCurly => write!(f, "RSquirly"),
            Token::LBracket => write!(f, "LBracket"),
            Token::RBracket => write!(f, "RBracket"),
            Token::Let => write!(f, "Let"),
            Token::Function => write!(f, "Function"),
            Token::Return => write!(f, "Return"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Eof => write!(f, "Eof"),
            Token::Illegal => write!(f, "Illegal"),
        }
    }
}

/// Valid datatypes
#[derive(Debug, PartialEq)]
pub enum LiteralType {
    /// 64 bit signed integer
    Int,
    /// String
    Str,
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::Int => write!(f, "Int"),
            LiteralType::Str => write!(f, "Str"),
        }
    }
}
