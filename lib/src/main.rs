use clap::Parser;
use lua::lexer::Lexer;

fn main() {
    let cli = cmd::Cli::parse();
    if let Some(source) = cli.read() {
        let mut lexer = Lexer::new(&source);
        for lexeme in lexer {
            println!("{:?}", lexeme);
        }
        println!("end: {}", &source.as_str()[source.len()-10..]);
    }
}
