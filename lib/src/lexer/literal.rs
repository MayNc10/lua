use std::collections::HashMap;
use lazy_static::lazy_static;
use strfmt::strfmt;
use regex::Regex;

use super::Token;

const SHORT_LITERAL_STR_RE_STR: &str = r#"\A(('(?<single_str>([^'\n\\]|\\.)*)')|("(?<double_str>([^"\n\\]|\\.)*)"))"#;
const LONG_STR_BEGIN_RE_STR: &str = r#"\A\[(?<equals>=*)\["#;
const LONG_STR_FORMAT_RE_STR: &str = r#"\A\[{0}\[\n?(?<str>(.|\s)*?)\]{0}\]"#; // using lazy capture isn't very efficient

/// These regexes have the issue that they need to match the general construction A|B|AB
/// But, we also need to seperate the parts into capture groups so that we can parse them together
/// So, we need to check the vacuosly true string
const DECIMAL_RE_STR: &str = r#"\A(?<base>[0-9]*)(?<fraction>\.[0-9]*)?([eE][-+]?[0-9]*)?"#;
const HEX_RE_STR: &str = r#"\A(?<hex>0[xX][0-9a-fA-F]*(?<fraction>\.[0-9a-fA-F]*)?(?<exp>[pP][-+]?[0-9]+)?)"#;

lazy_static! {
    static ref SHORT_LITERAL_STR_RE: Regex = Regex::new(SHORT_LITERAL_STR_RE_STR).expect("Error parsing short literal str regex");
    static ref LONG_STR_BEGIN_RE: Regex = Regex::new(LONG_STR_BEGIN_RE_STR).expect("Error parsing long literal str begining regex");
    static ref DECIMAL_RE: Regex = Regex::new(DECIMAL_RE_STR).expect("Error parsing decimal literal regex");
    static ref HEX_RE: Regex = Regex::new(HEX_RE_STR).expect("Error parsing hex literal regex");
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StringLiteralKind {
    Short,
    Long,
}

/// TODO: Figure out escape characters
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringLiteral {
    kind: StringLiteralKind,
    s: String,
    raw: String,
    // do we need the seperators as their own values
}

impl StringLiteral {
    pub fn match_long_str(s: &str) -> Option<StringLiteral> {
        // parse begining of string
        LONG_STR_BEGIN_RE.captures(s)
            .map(|captures| {
                let eq = &captures["equals"];
                // create format map
                let format_map = HashMap::from([(0.to_string(), eq)]);
                // create matching regex
                let format_re = Regex::new(
                    strfmt(LONG_STR_FORMAT_RE_STR, &format_map).expect("formatting string failed").as_str()
                ).expect("Error parsing long str format regex");
                format_re.captures(s)
            })
            .flatten()
            .map(|captures| {
                StringLiteral {
                    kind: StringLiteralKind::Long,
                    s: captures["str"].to_string(),
                    raw: captures[0].to_string(),
                }
            })
    }
    
    pub fn from_str(s: &str) -> Option<StringLiteral> {
        // try to match short string
        if let Some(capture) = SHORT_LITERAL_STR_RE.captures(s) {
            Some(StringLiteral { 
                kind: StringLiteralKind::Short, 
                s: capture
                    .name("single_str")
                    .unwrap_or_else(|| capture.name("double_str")
                        .expect("One of single_str, double_str should be captured"))
                    .as_str().to_string(), 
                raw: capture[0].to_string()
            })
        } else { StringLiteral::match_long_str(s) }
    }

    pub fn value(&self) -> &str { &self.s }
}

impl Token for StringLiteral {
    fn parse(text: &str) -> Option<(Self, usize)> {
        Self::from_str(text).map(|s| {
            let size = s.raw.len();
            (s, size)
        })
    }
    fn raw(&self) -> &str {
        &self.raw
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct NumericLiteral {
    value: f64,
    raw: String,
}

impl NumericLiteral {
    pub fn from_str(s: &str) -> Option<NumericLiteral> {
        // attempt to parse decimal
        DECIMAL_RE.captures(s)
            .map(|captures| {
                if captures.name("fraction").is_some_and(|s| s.len() > 1) || captures.name("base").is_some_and(|s| !s.is_empty()) {
                    Some(NumericLiteral { 
                        value: captures[0].parse().expect("Regex matched decimal float, but parsing failed"), 
                        raw: captures[0].to_string() 
                    })
                } else { None }
            })
            .flatten()
        .or_else(|| {
            HEX_RE.captures(s)
            .map(|captures| {
                // seperate and convert
                /* 
                let base = captures.name("hex")
                    .map(|m| i64::from_str_radix(&m.as_str()[2..], 16)
                        .expect("Regex match found base hex, but parsing failed")
                    );
                let fraction: Option<f64> = captures.name("fraction")
                    .map(|m| if m.is_empty() {0.0}
                        else {
                            i64::from_str_radix(&m.as_str()[1..], 16)
                                .expect("Regex match found fraction hex, but parsing failed")
                            as f64
                        }
                    )
                    .map(|f| {
                        // find where first nonzero is
                        let num_leading = captures.name("fraction")
                            .unwrap().as_str()[1..]
                            .find(|c: char| c.is_numeric() && c != '0').unwrap_or(0);
                        // then, for each space, we shift divide by 16
                        f / (16.0 * (num_leading as f64))
                    });
                let power: Option<f64> = captures.name("exp")
                    .map(|m| i64::from_str_radix(&m.as_str()[1..], 10)
                        .expect("Regex match found exp hex, but parsing failed")
                    )
                    .map(|e| 2.0f64.powi(e as i32) );
                
                if fraction.is_some() || base.is_some() {
                    if fraction.is_some() || power.is_some() {
                        let value = NumericValue::Integer(base.unwrap());
                        Some(NumericLiteral { value, raw: captures[0].to_string() })
                    } else { 
                        let value = NumericValue::Integer(base.unwrap());
                        Some(NumericLiteral { value, raw: captures[0].to_string() })
                    }
                } else { None }
                 */
                todo!("Hex lex!")
            }).flatten()
        })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn new(value: f64, raw: String) -> NumericLiteral {
        NumericLiteral { value, raw }
    }
}

impl Token for NumericLiteral {
    fn parse(text: &str) -> Option<(Self, usize)> {
        Self::from_str(text).map(|s| {
            let size = s.raw.len();
            (s, size)
        })
    }
    fn raw(&self) -> &str {
        &self.raw
    }
}
