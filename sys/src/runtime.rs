use crate::Value;

extern "C" {
    pub fn caml_main(argv: *const *const i8);
    pub fn caml_startup(argv: *const *const i8);
    pub fn caml_shutdown();
    pub fn caml_named_value(name: *const i8) -> *const Value;
}
