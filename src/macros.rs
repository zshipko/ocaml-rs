/// `frame!` can be used to create new local variables that play nicely with the garbage collector
#[macro_export]
macro_rules! frame {
    ($gc:ident: () $code:block) => {{
        $code
    }};
    ($gc:ident: ($($param:ident),+) $code:block) => {
        {
            struct __Values  {
               $($param: $crate::Value),*
            }

            $crate::interop::ocaml_frame!($gc, ($($param),+), {
                let (r, values) = {
                    $(
                        #[allow(unused_mut)]
                        #[allow(unused_assignments)]
                        let mut $param: $crate::Value = $crate::Value::unit();
                    )*
                    let r = $code;
                    (r, __Values { $(
                        $param,
                    )*})
                };


                $(
                    if values.$param != $crate::Value::unit() {
                        #[allow(unused_unsafe)]
                        unsafe { $param.keep_raw::<$crate::Value>(values.$param.0); }
                    }
                )*
                r
            })
        }
    }
}

#[cfg(not(feature = "no-std"))]
static PANIC_HANDLER_INIT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(feature = "no-std"))]
#[doc(hidden)]
pub fn init_panic_handler() {
    if PANIC_HANDLER_INIT
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_err()
    {
        return;
    }

    ::std::panic::set_hook(Box::new(|info| unsafe {
        let rt = crate::Runtime::recover_handle();
        let err = info.payload();
        let msg = if err.is::<&str>() {
            err.downcast_ref::<&str>().unwrap()
        } else if err.is::<String>() {
            err.downcast_ref::<String>().unwrap().as_ref()
        } else {
            "rust panic"
        };

        if let Some(err) = crate::Value::named("Rust_exception") {
            crate::Error::raise_value(&rt, err, msg);
        }

        crate::Error::raise_failure(&rt, msg)
    }))
}

/// `body!` is needed to help the OCaml runtime to manage garbage collection, it should
/// be used to wrap the body of each function exported to OCaml. Panics from Rust code
/// will automatically be unwound/caught here (unless the `no-std` feature is enabled)
///
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "C" fn example(a: ocaml::Value, b: ocaml::Value) -> ocaml::Value {
///     ocaml::body!(gc: (a, b) {
///         let a = a.int_val();
///         let b = b.int_val();
///         ocaml::Value::int(a + b)
///     })
/// }
/// ```
#[macro_export]
#[cfg(not(feature = "no-std"))]
macro_rules! body {
    ($gc:ident: $(())? $code:block) => {{
        #[allow(unused_variables)]
        let $gc = unsafe { $crate::Runtime::recover_handle() };

        // Ensure panic handler is initialized
        #[cfg(not(feature = "no-std"))]
        $crate::init_panic_handler();

        #[allow(unused_mut)]
        let mut r = |
            #[allow(unused_variables)]
            $gc: &mut $crate::Runtime
        | $code;
        r($gc)
    }};
    ($gc:ident: ($($param:ident),+) $code:block) => {{
        let $gc = unsafe { $crate::Runtime::recover_handle() };

        // Ensure panic handler is initialized
        #[cfg(not(feature = "no-std"))]
        $crate::init_panic_handler();

        struct __Values  {
           $($param: $crate::Value),*
        }

        let values = __Values { $(
            $param,
        )*};

        let ($($param),+) = $crate::interop::ocaml_frame!($gc, ($($param),+), {
            ($(
                unsafe { $crate::Value::new($param.keep_raw::<$crate::Value>(values.$param.0).get_raw()) }
            ),+)
        });

        #[allow(unused_mut)]
        let mut r = |$gc: &mut $crate::Runtime| $code;
        r($gc)
    }};
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
