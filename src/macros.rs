/// `local!` can used to define local variables in OCaml functions
#[macro_export]
macro_rules! local {
    ($($local:ident),*) => {
        #[allow(unused_mut)]
        $(let mut $local = $crate::Value($crate::sys::UNIT);)*
        #[allow(unused_unsafe)]
        $crate::sys::caml_param!($($local.0),*);
    }
}

/// `frame!` can be used to create new local variables that play nicely with the garbage collector
#[macro_export]
macro_rules! frame {
    (($($param:ident),*) $code:block) => {
       {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::sys::local_roots() };
            $crate::local!($($param),*);
            #[allow(unused_mut)]
            let mut res = || { $code };
            let res = res();

            #[allow(unused_unsafe)]
            unsafe { $crate::sys::set_local_roots(caml_frame) };
            res
        }
    }
}

/// `body!` is needed to help the OCaml runtime to manage garbage collection, it should
/// be used to wrap the body of each function exported to OCaml.
///
/// ```rust
/// #[no_mangle]
/// pub extern "C" fn example(a: ocaml::Value, b: ocaml::Value) -> ocaml::Value {
///     ocaml::body!((a, b) {
///         let a = a.int_val();
///         let b = b.int_val();
///         ocaml::Value::int(a + b)
///     })
/// }
/// ```
#[macro_export]
#[cfg(feature = "no-std")]
macro_rules! body {
    ($(($($param:expr),*))? $code:block) => {
        $crate::sys::caml_body!($(($($param.0),*))? $code);
    }
}

#[cfg(not(feature = "no-std"))]
static PANIC_HANDLER_INIT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(feature = "no-std"))]
#[doc(hidden)]
pub fn init_panic_handler() {
    if PANIC_HANDLER_INIT.compare_and_swap(false, true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    ::std::panic::set_hook(Box::new(|info| {
        let err = info.payload();
        let msg = if err.is::<&str>() {
            err.downcast_ref::<&str>().unwrap()
        } else if err.is::<String>() {
            err.downcast_ref::<String>().unwrap().as_ref()
        } else {
            "rust panic"
        };

        crate::Error::raise_failure(msg)
    }))
}

/// `body!` is needed to help the OCaml runtime to manage garbage collection, it should
/// be used to wrap the body of each function exported to OCaml. Panics from Rust code
/// will automatically be unwound/caught here (unless the `no-std` feature is enabled)
///
/// ```rust
/// #[no_mangle]
/// pub extern "C" fn example(a: ocaml::Value, b: ocaml::Value) -> ocaml::Value {
///     ocaml::body!((a, b) {
///         let a = a.int_val();
///         let b = b.int_val();
///         ocaml::Value::int(a + b)
///     })
/// }
/// ```
#[macro_export]
#[cfg(not(feature = "no-std"))]
macro_rules! body {
    ($(($($param:expr),*))? $code:block) => {{
        // Ensure panic handler is initialized
        $crate::init_panic_handler();

        // Initialize OCaml frame
        #[allow(unused_unsafe)]
        let caml_frame = unsafe { $crate::sys::local_roots() };

        // Initialize parameters
        $(
            $crate::sys::caml_param!($($param.0),*);
        )?

        // Execute Rust function
        #[allow(unused_mut)]
        let mut res = || {$code };
        let res = res();

        #[allow(unused_unsafe)]
        unsafe { $crate::sys::set_local_roots(caml_frame) };

        res
    }}
}

#[macro_export]
/// Convenience macro to create an OCaml array
macro_rules! array {
    ($($x:expr),*) => {{
        $crate::ToValue::to_value(&vec![$($crate::ToValue::to_value(&$x)),*])
    }}
}

#[macro_export]
/// Convenience macro to create an OCaml list
macro_rules! list {
    ($($x:expr),*) => {{
        let mut l = $crate::list::empty();
        for i in (&[$($x),*]).into_iter().rev() {
            $crate::list::push_hd(&mut l, $crate::ToValue::to_value(i));
        }
        l
    }};
}
