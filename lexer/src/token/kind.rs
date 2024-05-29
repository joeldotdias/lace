use std::fmt::Display;

/// Represents the parsed token type.
/// In case of a string or integer literal,
/// we store the datatype and value
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// Identifiers
    Ident {
        label: String,
    },

    /// Values allocated to a variable
    Literal {
        kind: LiteralKind,
        val: String,
    },

    // Operators
    /// =
    Assign,

    // Arithmetic
    /// +
    Plus,
    /// -
    Minus,
    /// /
    ForwardSlash,
    /// %
    Modulo,
    /// *
    Asterisk,

    // Logical
    /// !
    Bang,
    /// &&
    And,
    /// ||
    Or,

    // Relational
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    LessThan,
    /// >
    GreaterThan,
    /// <=
    LessThanEqual,
    /// >=
    GreaterThanEqual,

    // Delimiters
    /// .
    Dot,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LCurly,
    /// }
    RCurly,
    /// [
    LBracket,
    /// ]
    RBracket,

    // Keywords
    Let,
    Function,
    Return,
    If,
    Else,
    True,
    False,
    Source,

    // Comments
    /// //
    LineComment {
        content: String,
    },
    /// /* ... */
    BlockComment {
        content: String,
        terminated: bool,
    },

    /// Unknown or unrecognizable tokens.
    /// Includes emojis and other non ASCII characters.
    Illegal {
        ch: char,
    },

    /// End of input
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident { label } => write!(f, "Ident({label})"),
            TokenKind::Literal { kind, val } => write!(f, "{} Literal({})", kind, val),
            TokenKind::Assign => write!(f, "Assign"),
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Asterisk => write!(f, "Asterisk"),
            TokenKind::ForwardSlash => write!(f, "ForwardSlash"),
            TokenKind::Modulo => write!(f, "Modulo"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::NotEqual => write!(f, "NotEqual"),
            TokenKind::LessThan => write!(f, "LessThan"),
            TokenKind::GreaterThan => write!(f, "GreaterThan"),
            TokenKind::LessThanEqual => write!(f, "LessThanEqual"),
            TokenKind::GreaterThanEqual => write!(f, "GreaterThanEqual"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::LParen => write!(f, "Lparen"),
            TokenKind::RParen => write!(f, "Rparen"),
            TokenKind::LCurly => write!(f, "LSquirly"),
            TokenKind::RCurly => write!(f, "RSquirly"),
            TokenKind::LBracket => write!(f, "LBracket"),
            TokenKind::RBracket => write!(f, "RBracket"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Source => write!(f, "Source"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::True => write!(f, "True"),
            TokenKind::False => write!(f, "False"),
            TokenKind::LineComment { content } => write!(f, "LineComment {}", content),
            TokenKind::BlockComment {
                content,
                terminated: _,
            } => write!(f, "BlockComment {}", content),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Illegal { ch } => write!(f, "Illegal({ch})"),
        }
    }
}

impl TokenKind {
    pub fn try_keyword(label: &str) -> Option<TokenKind> {
        let keyword = match label {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "return" => TokenKind::Return,
            "source" => TokenKind::Source,
            _ => return None,
        };

        Some(keyword)
    }
}

/// Valid datatypes.
/// Booleans are just true and false tokens.
/// They are put into literal nodes while constructing the AST
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    /// 64 bit signed integer
    Int,
    /// 64 bit signed floating point number
    Float,
    /// Characters
    Char { terminated: bool },
    /// String
    Str { terminated: bool },
}

impl Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKind::Int => write!(f, "Int"),
            LiteralKind::Float => write!(f, "Float"),
            LiteralKind::Char { terminated } => match terminated {
                true => write!(f, "Char"),
                false => write!(f, "Unterm Char"),
            },
            LiteralKind::Str { terminated } => match terminated {
                true => write!(f, "Str"),
                false => write!(f, "Unterm Str"),
            },
        }
    }
}
