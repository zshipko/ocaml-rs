use core;
use core::mlvalues::Value;

/// Returns a named value registered by OCaml
pub fn named_value<S: AsRef<str>>(name: S) -> Option<Value> {
    unsafe {
        let named = core::callback::caml_named_value(name.as_ref().as_ptr());
        if named.is_null() {
            return None
        }

        Some(*named)
    }
}
