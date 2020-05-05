use crate::{Char, Value};

extern "C" {
    pub fn caml_main(argv: *const *const Char);
    pub fn caml_startup(argv: *const *const Char);
    pub fn caml_shutdown();
    pub fn caml_named_value(name: *const Char) -> *const Value;
}
