use crate::lexer::Lexer;

pub trait Statement {}
pub struct EmptyStatement {}
pub struct Assignment {}
pub struct WhileStatement {}
pub struct RepeatStatement {}
pub struct Conditional {}
pub struct Goto {}
pub struct Label {}
pub struct ForStatement {}
pub struct FunctionCall {}
pub struct FunctionDef {}
pub struct Break {} // ?
pub struct Return {} // ?

pub fn parse_statement(lex: &mut Lexer) -> Option<Box<dyn Statement>> {
    // parse assignment
    // parse if
    // parse functioncall
    // parse functiondef

    // parse return
    todo!()
}