use std::{fs::read_to_string, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub path: Option<PathBuf>,
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>
}

impl Cli {
    // TODO: Errors!
    pub fn read(&self) -> Option<String> {
        if let Some(path) = &self.path {
            read_to_string(path).ok()
        } else { None }
    }

    pub fn args(&self) -> Vec<(i64, String)> {
        let mut idx = 1;
        let mut v = self.args.iter().map(|s| {
            idx += 1;
            (idx - 1, s.to_string())
        }).collect::<Vec<_>>();
        if let Some(p) = self.path.as_ref() {
            v.push((0, p.to_str().unwrap().to_string()));
        }
        v
    }
}