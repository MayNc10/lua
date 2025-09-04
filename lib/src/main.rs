use clap::Parser;
use lua::{ast::context::Ctx, lexer::Lexer, parser::parse};

fn main() {
    let cli = cmd::Cli::parse();
    if let Some(source) = cli.read() {
        let block = parse(&source).unwrap();
        let mut context = Ctx::new();
        block.walk(&mut context);
    }
}
