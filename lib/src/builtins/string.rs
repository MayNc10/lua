use std::ffi::c_char;

use crate::{ast::expression::Expression, value::Value};

pub fn format(mut fmt: &str, args: &Vec<Value>) -> String {
    let mut buf = String::new();
    let mut iter = args.iter();
    while let Some(idx) = fmt.find('%') {
        buf.push_str(&fmt[..idx]);
        fmt = &fmt[idx + 1..];
        match &fmt[0..1] {
            "d" | "i" => {
                buf = format!("{buf}{}", iter.next().and_then(|v| v.as_number().map(|n| n as i64)).expect("failed to parse as number"))
            },
            "u" => {
                buf = format!("{buf}{}", iter.next().and_then(|v| v.as_number().map(|n| n as u64)).expect("failed to parse as number"))
            },
            "o" => {
                buf = format!("{buf}{:o}", iter.next().and_then(|v| v.as_number().map(|n| n as i64)).expect("failed to parse as number"))
            },
            "x" => {
                buf = format!("{buf}{:x}", iter.next().and_then(|v| v.as_number().map(|n| n as i64)).expect("failed to parse as number"))
            },
            "X" => {
                buf = format!("{buf}{:X}", iter.next().and_then(|v| v.as_number().map(|n| n as i64)).expect("failed to parse as number"))
            },
            "f" => {
                buf = format!("{buf}{}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
            },
            "e" => {
                buf = format!("{buf}{:e}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
            },
            "E" => {
                buf = format!("{buf}{:E}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
            },
            "g" => {
                buf = format!("{buf}{:?}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
            },
            "G" => {
                // FIXME
                buf = format!("{buf}{:?}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
            },
            "c" => {
                buf = format!("{buf}{}", iter.next()
                    .and_then(|v| v.as_number().map(|c| ((c as u64) % 256) as u8 as char)).expect("failed to parse as number"))
            },
            "s" => {
                buf = format!("{buf}{}", iter.next().and_then(|v| v.as_string()).expect("failed to parse as string"))
            },
            "." => { 
                fmt = &fmt[1..];
                let mut idx = 0;
                while fmt[idx..idx + 1].chars().next().unwrap().is_numeric() {
                    idx += 1;
                }
                let num = fmt[0..idx].parse::<usize>().expect("number wasn't found after dot");
                fmt = &fmt[idx..];
                match &fmt[0..1] {
                    "e" => {
                        buf = format!("{buf}{:.*e}", num, iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
                    },
                    "E" => {
                        buf = format!("{buf}{:.*E}", num, iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"))
                    },
                    "g" => {
                        // FIXME
                        let formatted = format!("{:?}", iter.next().and_then(|v| v.as_number()).expect("failed to parse as number"));
                        buf = format!("{buf}{:.*}", num + 1, formatted);
                    },
                    _ => todo!(),
                }
            },
            _ => todo!()
        }
        fmt = &fmt[1..];
    }
    buf.push_str(fmt);
    buf
}