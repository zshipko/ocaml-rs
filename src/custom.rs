use crate::*;

/// CustomOps duplicates `sys::custom::custom_operations` to provide a slightly nicer experience in
/// Rust
///
/// This should rarely be constructed manually, `custom!` simplifies the process of creating custom
/// types.
///
/// See [the struct
/// custom_operations](https://caml.inria.fr/pub/docs/manual-ocaml/intfc.html#ss:c-custom-ops)
/// section in the OCaml manual for more information about each field
#[derive(Clone)]
#[repr(C)]
#[allow(missing_docs)]
pub struct CustomOps {
    pub identifier: *const i8,
    pub finalize: Option<unsafe extern "C" fn(v: Value)>,
    pub compare: Option<unsafe extern "C" fn(v1: Value, v2: Value) -> i32>,
    pub hash: Option<unsafe extern "C" fn(v: Value) -> Int>,

    pub serialize: Option<unsafe extern "C" fn(v: Value, bsize_32: *mut Uint, bsize_64: *mut Uint)>,
    pub deserialize: Option<unsafe extern "C" fn(dst: *mut core::ffi::c_void) -> Uint>,
    pub compare_ext: Option<unsafe extern "C" fn(v1: Value, v2: Value) -> i32>,
    pub fixed_length: *const sys::custom_fixed_length,
}

impl Default for CustomOps {
    fn default() -> CustomOps {
        CustomOps {
            identifier: core::ptr::null(),
            finalize: None,
            compare: None,
            hash: None,
            serialize: None,
            deserialize: None,
            compare_ext: None,
            fixed_length: core::ptr::null_mut(),
        }
    }
}

/// CustomType wraps `CustomOps` to provide `name` and `fixed_length` in the safe manner
pub struct CustomType {
    /// Type name
    pub name: &'static str,
    /// Owned `fixed_length` value
    pub fixed_length: Option<sys::custom_fixed_length>,
    /// Callbacks
    pub ops: CustomOps,
}

/// `Custom` is used to define OCaml types that wrap existing Rust types, but are owned by the
/// garbage collector
///
/// A custom type can only be converted to a `Value` using `ToValue`, but can't be converted from a
/// value. Once the Rust value is owned by OCaml it should be accessed using `ocaml::Pointer` to
/// avoid reallocating the same value
///
/// ```rust
/// struct Example(ocaml::Int);
///
/// ocaml::custom! (Example);
///
/// #[cfg(feature = "derive")]
/// #[ocaml::func]
/// pub unsafe fn example() -> Example {
///     Example(123)
/// }
///
/// #[cfg(feature = "derive")]
/// #[ocaml::func]
/// pub unsafe fn example_value(x: ocaml::Pointer<Example>) -> ocaml::Int {
///     x.as_ref().0
/// }
/// ```
pub trait Custom {
    /// Custom type implementation
    const TYPE: CustomType;

    /// `used` parameter to `alloc_custom`. This helps determine the frequency of garbage
    /// collection related to this custom type.
    const USED: usize = 0;

    /// `max` parameter to `alloc_custom`. This helps determine the frequency of garbage collection
    /// related to this custom type
    const MAX: usize = 1;

    /// Get a static reference the this type's `CustomOps` implementation
    fn ops() -> &'static CustomOps {
        Self::TYPE.ops.identifier = Self::TYPE.name.as_ptr() as *const i8;
        if let Some(x) = Self::TYPE.fixed_length {
            Self::TYPE.ops.fixed_length = &x;
        }

        &Self::TYPE.ops
    }
}

unsafe impl<T: 'static + Custom> ToValue for T {
    fn to_value(self) -> Value {
        let val: crate::Pointer<T> = Pointer::alloc_custom(self);
        val.to_value()
    }
}

/// Create a custom OCaml type from an existing Rust type
///
/// See [the struct
/// custom_operations](https://caml.inria.fr/pub/docs/manual-ocaml/intfc.html#ss:c-custom-ops)
/// section in the OCaml manual for more information about each field
///
/// ```rust
/// struct MyType {
///     s: String,
///     i: i32,
/// }
///
/// extern "C" fn mytype_finalizer(_: ocaml::Value) {
///     println!("This runs when the value gets garbage collected");
/// }
///
/// extern "C" fn mytype_compare(a: ocaml::Value, b: ocaml::Value) -> i32 {
///     let a: ocaml::Pointer::<MyType> = ocaml::FromValue::from_value(a);
///     let b: ocaml::Pointer::<MyType> = ocaml::FromValue::from_value(b);
///
///     let a_i = a.as_ref().i;
///     let b_i = b.as_ref().i;
///
///     if a_i == b_i {
///         return 0
///     }
///
///     if a_i < b_i {
///         return -1;
///     }
///
///     1
/// }
///
/// ocaml::custom!(MyType {
///     finalize: mytype_finalizer,
///     compare: mytype_compare,
/// });
///
/// // This is equivalent to
/// struct MyType2 {
///     s: String,
///     i: i32,
/// }
///
/// impl ocaml::Custom for MyType2 {
///     const TYPE: ocaml::custom::CustomType = ocaml::custom::CustomType {
///         name: "rust.MyType\0",
///         fixed_length: None,
///         ops: ocaml::custom::CustomOps {
///             identifier: core::ptr::null(), // This will be filled in when the struct is used
///             fixed_length: core::ptr::null_mut(), // This will be filled in too
///             finalize: Some(mytype_finalizer),
///             compare: Some(mytype_compare),
///             compare_ext: None,
///             deserialize: None,
///             hash: None,
///             serialize: None
///         }
///     };
/// }
/// ```
///
/// Additionally, `custom` can be used inside the `impl` block:
///
/// ```rust
/// extern "C" fn implexample_finalizer(_: ocaml::Value) {
///     println!("This runs when the value gets garbage collected");
/// }
///
/// struct ImplExample<'a>(&'a str);
///
/// impl<'a> ocaml::Custom for ImplExample<'a> {
///     ocaml::custom! {
///         name: "rust.ImplExample",
///         finalize: implexample_finalizer
///     }
/// }
///
/// // This is equivalent to:
///
/// struct ImplExample2<'a>(&'a str);
///
/// ocaml::custom!(ImplExample2<'a> {
///     finalize: implexample_finalizer,
/// });
/// ```
#[macro_export]
macro_rules! custom {
    ($name:ident $(<$t:tt>)? $({$($k:ident : $v:expr),* $(,)? })?) => {
        impl $(<$t>)? $crate::Custom for $name $(<$t>)? {
            $crate::custom! {
                name: concat!("rust.", stringify!($name))
                $(, $($k: $v),*)?
            }
        }
    };
    {name : $name:expr $(, $($k:ident : $v:expr),*)? $(,)? } => {
        const TYPE: $crate::custom::CustomType = $crate::custom::CustomType {
            name: concat!($name, "\0"),
            fixed_length: None,
            ops: $crate::custom::CustomOps {
                $($($k: Some($v),)*)?
                .. $crate::custom::CustomOps {
                    identifier: core::ptr::null(),
                    fixed_length: core::ptr::null_mut(),
                    compare: None,
                    compare_ext: None,
                    deserialize: None,
                    finalize: None,
                    hash: None,
                    serialize: None,
                }
            },
        };
    };
}
