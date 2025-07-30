use lazy_static::lazy_static;
use regex::Regex;

use super::Token;

const IDENTIFIER_RE_STR: &str = r#"(?m)\A[a-zA-Z_][a-zA-Z0-9_]*"#;

lazy_static! {
    static ref IDENTIFIER_RE: Regex = Regex::new(IDENTIFIER_RE_STR).expect("Identifier regex failed to parse");
}

#[derive(Clone, PartialEq, Debug)]
pub struct Identifier(pub String);

impl Token for Identifier {
    fn parse(text: &str) -> Option<(Self, usize)> {
        if let Some(captures) = IDENTIFIER_RE.captures(text) {
            Some((Identifier(captures[0].to_string()), captures[0].len()))
        } else { None }
    }
    fn raw(&self) -> &str {
        self.0.as_str()
    }
}