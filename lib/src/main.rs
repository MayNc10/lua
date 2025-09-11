use clap::Parser;
use lua::{ast::context::Ctx, builtins::prelude, lexer::Lexer, parser::parse};

fn main() {
    let cli = cmd::Cli::parse();
    if let Some(source) = cli.read() {
        let block = parse(&source).unwrap();
        let mut context = Ctx::new();
        prelude(&mut context);
        block.walk(&mut context);
    }
}
