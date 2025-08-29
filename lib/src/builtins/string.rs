use std::ffi::c_char;

use crate::ast::expression::Expression;

unsafe extern "C"
{
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...);
}

pub fn format(fmt: &str, args: &Vec<Expression>) -> String {

}