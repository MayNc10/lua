use std::{fs::read_to_string, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub path: Option<PathBuf>,
}

impl Cli {
    // TODO: Errors!
    pub fn read(&self) -> Option<String> {
        if let Some(path) = &self.path {
            read_to_string(path).ok()
        } else { None }
    }
}