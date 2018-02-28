use std::ffi::CString;

use core::mlvalues::Value;
use core::custom;
use error::Error;

pub fn custom<S: AsRef<str>>(name: S, finalize: ::std::option::Option<unsafe extern "C" fn(v: Value)>) -> Result<(), Error> {
    let c = match CString::new(name.as_ref()) {
        Ok(c) => c,
        Err(_) => return Err(Error::InvalidCString)
    };

    let mut ops = custom::CustomOperations {
        identifier: c.as_ptr() as *mut i8,
        finalize: finalize,
        compare: None,
        hash: None,
        serialize: None,
        deserialize: None,
        compare_ext: None
    };

    unsafe {
        custom::caml_register_custom_operations(&mut ops);
    }

    Ok(())
}
