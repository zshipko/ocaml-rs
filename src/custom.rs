use core::mlvalues::Value;
use core::custom;

pub fn custom<S: AsRef<str>>(name: S, finalize: ::std::option::Option<unsafe extern "C" fn(v: Value)>) {
    let mut ops = custom::CustomOperations {
        identifier: name.as_ref().as_ptr() as *mut i8,
        finalize: finalize,
        compare: None,
        hash: None,
        serialize: None,
        deserialize: None,
        compare_ext: None
    };

    unsafe {
        custom::caml_register_custom_operations(&mut ops)
    }
}
