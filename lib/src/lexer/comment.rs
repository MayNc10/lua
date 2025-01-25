use lazy_static::lazy_static;
use regex::{self, Regex};
use super::*;

const RE_STR: &str = r"\A(--\[\[(?<multi>[^\[\]]*)\]\]|-(?<single>.*)$)";
const MULTILINE_GROUP: &str = "multi";
const SINGLELINE_GROUP: &str = "single";

lazy_static! {
    static ref RE: Regex = Regex::new(RE_STR).expect("Error parsing comment regex");
}

pub struct Comment {
    comment: String,
}

impl Token for Comment {
    fn parse(text: &str) -> Option<(Self, usize)> {
        // try to match comment regex
        if let Some(captures) = RE.captures(text) {
            let len = captures[0].len(); // how many characters do we consume in total?
            let comment = {if let Some(multi_line) = captures.name(MULTILINE_GROUP) {
                multi_line.as_str()
            } else {
                &captures[SINGLELINE_GROUP]
            }}.to_string();
            Some((Comment { comment }, len))
        }
        else { None }
    }
    fn raw(&self) -> &str {
        self.comment.as_str()
    }
}