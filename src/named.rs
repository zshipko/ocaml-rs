use std::ffi::CString;

use crate::sys;
use crate::value::Value;

/// Returns a named value registered by OCaml
pub fn named_value<S: AsRef<str>>(name: S) -> Option<Value> {
    unsafe {
        let s = match CString::new(name.as_ref()) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let named = sys::callback::caml_named_value(s.as_ptr() as *const u8);
        if named.is_null() {
            return None;
        }

        Some(Value::new(*named))
    }
}
