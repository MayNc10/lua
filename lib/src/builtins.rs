use crate::{ast::context::Ctx, lexer::identifier::Identifier, value::Value};

pub mod io;
pub mod math;
pub mod string;

pub fn prelude(ctx: &mut Ctx) {
    let io_table = io::create_io_table();
    ctx.new_global(Identifier("io".to_string()), Value::Table(io_table));
}