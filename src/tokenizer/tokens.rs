use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
    StringLiteral(String),
    InterpolatedString(String),

    //Operators
    //Arithmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,

    // Increment and Decrement Operators
    // Increment,
    // Decrement,

    // Assignment Operators
    Assignment,

    // Comparison Operators
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Logical Operators
    And,
    Or,
    Not,

    // Bitwise Operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    // Punctuation
    SingleQuote,
    DoubleQuote,
    Backtick,
    Comma,
    Semicolon,
    Colon,
    Dot,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Whitespace(String),
    Comment(String),
    EOF,
}

impl Token {
    pub fn is_ignorable(&self) -> bool {
        matches!(self, Token::Whitespace(_) | Token::Comment(_))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "NUMBER {} {}", n, n.parse::<f64>().unwrap_or(0.0)),
            Token::Identifier(id) => write!(f, "IDENTIFIER {} null", id),
            Token::StringLiteral(s) => write!(f, "STRING \"{}\" {}", s, s),
            Token::InterpolatedString(s) => write!(f, "InterpolatedString({})", s),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Multiply => write!(f, "STAR * null"),
            Token::Divide => write!(f, "SLASH / null"),
            Token::Modulus => write!(f, "MODULUS % null"),
            // Token::Increment => write!(f, "INCREMENT ++ null"),
            // Token::Decrement => write!(f, "DECREMENT -- null"),
            Token::Assignment => write!(f, "EQUAL = null"),
            Token::Equals => write!(f, "EQUAL_EQUAL == null"),
            Token::NotEquals => write!(f, "BANG_EQUAL != null"),
            Token::GreaterThan => write!(f, "GREATER > null"),
            Token::LessThan => write!(f, "LESS < null"),
            Token::GreaterThanOrEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::LessThanOrEqual => write!(f, "LESS_EQUAL <= null"),
            Token::And => write!(f, "AND && null"),
            Token::Or => write!(f, "OR || null"),
            Token::Not => write!(f, "NOT ! null"),
            Token::BitwiseAnd => write!(f, "BITWISE_AND & null"),
            Token::BitwiseOr => write!(f, "BITWISE_OR | null"),
            Token::BitwiseXor => write!(f, "BITWISE_XOR ^ null"),
            Token::BitwiseNot => write!(f, "BitwiseNot"),
            Token::BitwiseShiftLeft => write!(f, "BITWISE_SHIFT_LEFT << null"),
            Token::BitwiseShiftRight => write!(f, "BITWISE_SHIFT_RIGHT >> null"),
            Token::SingleQuote => write!(f, "SINGLE_QUOTE ' null"),
            Token::DoubleQuote => write!(f, "DOUBLE_QUOTE \" null"),
            Token::Backtick => write!(f, "BACKTICK ` null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Colon => write!(f, "COLON : null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Token::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::LeftBracket => write!(f, "LEFT_BRACKET [ null"),
            Token::RightBracket => write!(f, "RIGHT_BRACKET ] null"),
            Token::Whitespace(ws) => write!(f, ""), // we don't print whitespace tokens
            Token::Comment(comment) => write!(f, "COMMENT {} null", comment),
            Token::EOF => write!(f, "EOF  null"),
        }
    }
}
