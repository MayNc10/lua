use super::Token;

/// TODO: Reuse Operator code
#[derive(Clone, Debug, PartialEq)]
pub enum Seperator {
    DoubleColon,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Dot,
    Semicolon,
    Comma,
    Colon,
}

impl Token for Seperator {
    fn parse(text: &str) -> Option<(Self, usize)> {
        if &text[0..2] == "::" { 
            return Some((Self::DoubleColon, 2));
        }
        match &text[0..1] {
            "[" => Some((Self::OpenBracket, 1)),
            "]" => Some((Self::CloseBracket, 1)),
            "(" => Some((Self::OpenParen, 1)),
            ")" => Some((Self::CloseParen, 1)),
            "{" => Some((Self::OpenCurly, 1)),
            "}" => Some((Self::CloseCurly, 1)),
            "." => Some((Self::Dot, 1)),
            ";" => Some((Self::Semicolon, 1)),
            "," => Some((Self::Comma, 1)),
            ":" => Some((Self::Colon, 1)),
            _ => None,
        }
    }
    fn raw(&self) -> &str {
        match self {
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenCurly => "{",
            Self::CloseCurly => "}",
            Self::Dot => ".",
            Self::Semicolon => ";",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::DoubleColon => "::",
        }
    }
}