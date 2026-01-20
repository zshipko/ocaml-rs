use crate::{Char, Value};

unsafe extern "C" {
    pub fn caml_format_exception(v: Value) -> *const Char;
}
