use core;
use value::Value;

/// Returns a named value registered by OCaml
pub fn named_value<S: AsRef<str>>(name: S) -> Option<Value> {
    unsafe {
        let p = format!("{}\0", name.as_ref());
        let named = core::callback::caml_named_value(p.as_str().as_ptr());
        if named.is_null() {
            return None;
        }

        Some(Value::new(*named))
    }
}
