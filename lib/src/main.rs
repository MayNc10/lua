use clap::Parser;
use lua::{lexer::Lexer, parser::parse};

fn main() {
    let cli = cmd::Cli::parse();
    if let Some(source) = cli.read() {
        let mut lexer = Lexer::new(&source);
        for lexeme in lexer {
            //println!("{:?}", lexeme);
        }
        let block = parse(&source);
    }
}
