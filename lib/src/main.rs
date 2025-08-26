use clap::Parser;
use lua::{ast::context::Ctx, lexer::Lexer, parser::parse};

fn main() {
    let cli = cmd::Cli::parse();
    if let Some(source) = cli.read() {
        let mut lexer = Lexer::new(&source);
        for lexeme in lexer {
            //println!("{:?}", lexeme);
        }
        let block = parse(&source).unwrap();
        //println!("\n\n\n\n\n\n");
        println!("parsed ast:");
        block.print_tree(0);

        let mut context = Ctx::new();
        block.walk(&mut context);
    }
}
