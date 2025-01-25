use lazy_static::lazy_static;
use regex::Regex;

use super::Token;

const WHITESPACE_RE_STR: &str = r#"(?m)\A\s+"#;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(WHITESPACE_RE_STR).expect("Error parsing whitespace regex");
}

#[derive(Clone, PartialEq, Debug)]
pub struct Whitespace(String);

impl Token for Whitespace {
    fn parse(text: &str) -> Option<(Self, usize)> {
        if let Some(captures) = WHITESPACE_RE.captures(text) {
            Some((Whitespace(captures[0].to_string()), captures[0].len()))
        } else { None }
    }
    fn raw(&self) -> &str {
        self.0.as_str()
    }
}