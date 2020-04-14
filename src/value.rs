use crate::error::{CamlError, Error};
use crate::sys;
use crate::tag::Tag;

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = sys::Size;

/// Value wraps the native OCaml `value` type transparently, this means it has the
/// same representation as an `ocaml_sys::Value`
#[derive(Debug, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value(pub sys::Value);

impl Clone for Value {
    fn clone(&self) -> Value {
        Value(self.0)
    }
}

/// `ToValue` is used to convert from Rust types to OCaml values
pub unsafe trait ToValue {
    /// Convert to OCaml value
    fn to_value(self) -> Value;
}

/// `FromValue` is used to convert from OCaml values to Rust types
pub unsafe trait FromValue {
    /// Convert from OCaml value
    fn from_value(v: Value) -> Self;
}

unsafe impl ToValue for Value {
    fn to_value(self) -> Value {
        Value(self.0)
    }
}

unsafe impl FromValue for Value {
    #[inline]
    fn from_value(v: Value) -> Value {
        v
    }
}

const NONE: Value = Value(sys::val_int(0));
const UNIT: Value = Value(sys::UNIT);

impl Value {
    /// Returns a named value registered by OCaml
    pub fn named<T: FromValue>(name: &str) -> Option<T> {
        unsafe {
            let s = match std::ffi::CString::new(name) {
                Ok(s) => s,
                Err(_) => return None,
            };
            let named = sys::caml_named_value(s.as_ptr());
            if named.is_null() {
                return None;
            }

            Some(FromValue::from_value(Value(*named)))
        }
    }

    /// Allocate a new value with the given size and tag.
    pub fn alloc(n: usize, tag: Tag) -> Value {
        crate::frame!((x) {
            x = Value(unsafe { sys::caml_alloc(n, tag.into()) });
            x
        })
    }

    /// Allocate a new tuple value
    pub fn alloc_tuple(n: usize) -> Value {
        crate::frame!((x) {
            x = Value(unsafe { sys::caml_alloc_tuple(n) });
            x
        })
    }

    /// Allocate a new small value with the given size and tag
    pub fn alloc_small(n: usize, tag: Tag) -> Value {
        crate::frame!((x) {
            x = Value(unsafe { sys::caml_alloc_small(n, tag.into()) });
            x
        })
    }

    /// Allocate a new value with a finalizer
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Poiner::alloc_custom` when possible.
    pub fn alloc_final<T>(
        finalizer: unsafe extern "C" fn(Value),
        cfg: Option<(usize, usize)>,
    ) -> Value {
        let (used, max) = cfg.unwrap_or_else(|| (0, 1));
        crate::frame!((x) {
            unsafe {
                Value(sys::caml_alloc_final(
                    std::mem::size_of::<T>(),
                    std::mem::transmute(finalizer),
                    used,
                    max
                ))
            }
        })
    }

    /// Allocate custom value
    pub fn alloc_custom<T: crate::Custom>() -> Value {
        let size = std::mem::size_of::<T>();
        crate::frame!((x) {
            unsafe {
                x = Value(sys::caml_alloc_custom(T::ops() as *const _ as *const sys::custom_operations, size, T::USED, T::MAX));
                x
            }
        })
    }

    /// Create a new Value from an existing OCaml `value`
    #[inline]
    pub const fn new(v: sys::Value) -> Value {
        Value(v)
    }

    /// Get array length
    pub fn array_length(self) -> usize {
        unsafe { sys::caml_array_length(self.0) }
    }

    /// See caml_register_global_root
    pub fn register_global_root(&mut self) {
        unsafe { sys::caml_register_global_root(&mut self.0) }
    }

    /// Set caml_remove_global_root
    pub fn remove_global_root(&mut self) {
        unsafe { sys::caml_remove_global_root(&mut self.0) }
    }

    /// Get the tag for the underlying OCaml `value`
    pub fn tag(self) -> Tag {
        unsafe { sys::tag_val(self.0).into() }
    }

    /// Convert a boolean to OCaml value
    pub const fn bool(b: bool) -> Value {
        Value::int(b as crate::Int)
    }

    /// OCaml Some value
    pub fn some<V: ToValue>(v: V) -> Value {
        crate::frame!((x) {
            unsafe {
                x = Value(sys::caml_alloc(1, 0));
                x.store_field(0, v.to_value());
            }
            x
        })
    }

    /// OCaml None value
    #[inline(always)]
    pub const fn none() -> Value {
        NONE
    }

    /// OCaml Unit value
    #[inline(always)]
    pub const fn unit() -> Value {
        UNIT
    }

    /// Create a variant value
    pub fn variant(tag: u8, value: Option<Value>) -> Value {
        crate::frame!((x) {
            match value {
                Some(v) => unsafe {
                    x = Value(sys::caml_alloc(1, tag));
                    x.store_field(0, v)
                },
                None => x = Value(unsafe { sys::caml_alloc(0, tag) }),
            }
            x
        })
    }

    /// Create a new opaque pointer Value
    pub fn ptr<T>(p: *const T) -> Value {
        Value(p as sys::Value)
    }

    /// Create an OCaml `int`
    pub const fn int(i: crate::Int) -> Value {
        Value(sys::val_int(i))
    }

    /// Create an OCaml `int`
    pub const fn uint(i: crate::Uint) -> Value {
        Value(sys::val_int(i as crate::Int))
    }

    /// Create an OCaml `Int64` from `i64`
    pub fn int64(i: i64) -> Value {
        crate::frame!((x) {
            unsafe { x = Value(sys::caml_copy_int64(i)) };
            x
        })
    }

    /// Create an OCaml `Int32` from `i32`
    pub fn int32(i: i32) -> Value {
        crate::frame!((x) {
            unsafe { x = Value(sys::caml_copy_int32(i)) };
            x
        })
    }

    /// Create an OCaml `Nativeint` from `isize`
    pub fn nativeint(i: isize) -> Value {
        frame!((x) {
            unsafe { x = Value(sys::caml_copy_nativeint(i)) };
            x
        })
    }

    /// Create an OCaml `Float` from `f64`
    pub fn float(d: f64) -> Value {
        frame!((x) {
            unsafe { x = Value(sys::caml_copy_double(d)) }
            x
        })
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub fn is_block(self) -> bool {
        sys::is_block(self.0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub fn is_long(self) -> bool {
        sys::is_long(self.0)
    }

    /// Get index of underlying OCaml block value
    pub fn field<T: FromValue>(self, i: Size) -> T {
        unsafe { T::from_value(Value(*sys::field(self.0, i))) }
    }

    /// Set index of underlying OCaml block value
    pub fn store_field<V: ToValue>(&mut self, i: Size, val: V) {
        unsafe { sys::store_field(self.0, i, val.to_value().0) }
    }

    /// Convert an OCaml `int` to `isize`
    pub const fn int_val(self) -> isize {
        sys::int_val(self.0)
    }

    /// Convert an OCaml `Float` to `f64`
    pub fn float_val(self) -> f64 {
        unsafe { *self.ptr_val::<f64>() }
    }

    /// Convert an OCaml `Int32` to `i32`
    pub fn int32_val(self) -> i32 {
        unsafe { *self.custom_ptr_val::<i32>() }
    }

    /// Convert an OCaml `Int64` to `i64`
    pub fn int64_val(self) -> i64 {
        unsafe { *self.custom_ptr_val::<i64>() }
    }

    /// Convert an OCaml `Nativeint` to `isize`
    pub fn nativeint_val(self) -> isize {
        unsafe { *self.custom_ptr_val::<isize>() }
    }

    /// Get pointer to data stored in an OCaml custom value
    pub fn custom_ptr_val<T>(self) -> *const T {
        unsafe { sys::field(self.0, 1) as *const T }
    }

    /// Get mutable pointer to data stored in an OCaml custom value
    pub fn custom_mut_ptr_val<T>(self) -> *mut T {
        unsafe { sys::field(self.0, 1) as *mut T }
    }

    /// Get pointer to data stored in an opaque value
    pub fn ptr_val<T>(self) -> *const T {
        self.0 as *const T
    }

    /// Get mutable pointer to data stored in an opaque value
    pub fn mut_ptr_val<T>(self) -> *mut T {
        self.0 as *mut T
    }

    /// Extract OCaml exception
    pub fn exception<A: FromValue>(self) -> Option<A> {
        if !self.is_exception_result() {
            return None;
        }

        Some(A::from_value(Value(crate::sys::extract_exception(self.0))))
    }

    /// Call a closure with a single argument, returning an exception value
    pub fn call<A: ToValue>(self, arg: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!((res) {
            res = unsafe { Value(sys::caml_callback_exn(self.0, arg.to_value().0)) };
            res
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with two arguments, returning an exception value
    pub fn call2<A: ToValue, B: ToValue>(self, arg1: A, arg2: B) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!((res) {
            res = unsafe {
                Value(sys::caml_callback2_exn(self.0, arg1.to_value().0, arg2.to_value().0))
            };
            res
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with three arguments, returning an exception value
    pub fn call3<A: ToValue, B: ToValue, C: ToValue>(
        self,
        arg1: A,
        arg2: B,
        arg3: C,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!((res) {
            res = unsafe {
                Value(sys::caml_callback3_exn(
                    self.0,
                    arg1.to_value().0,
                    arg2.to_value().0,
                    arg3.to_value().0,
                ))
            };
            res
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with `n` arguments, returning an exception value
    pub fn call_n<A: AsRef<[Value]>>(self, args: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.as_ref().len();
        let x: Vec<sys::Value> = args.as_ref().iter().map(|x| x.0).collect();

        let mut v = crate::frame!((res) {
            res = unsafe {
                Value(sys::caml_callbackN_exn(
                    self.0,
                    n,
                    x.as_ptr() as *mut sys::Value,
                ))
            };
            res
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Modify an OCaml value in place
    pub fn modify<V: ToValue>(&mut self, v: V) {
        unsafe { sys::caml_modify(&mut self.0, v.to_value().0) }
    }

    /// Determines if the current value is an exception
    pub fn is_exception_result(self) -> bool {
        crate::sys::is_exception_result(self.0)
    }

    /// Get hash variant as OCaml value
    pub fn hash_variant<S: AsRef<str>>(name: S, a: Option<Value>) -> Value {
        let s = std::ffi::CString::new(name.as_ref()).expect("Invalid C string");
        let hash = unsafe { Value(sys::caml_hash_variant(s.as_ptr() as *const u8)) };
        match a {
            Some(x) => {
                let mut output = Value::alloc_small(2, Tag(0));
                output.store_field(0, hash);
                output.store_field(1, x);
                output
            }
            None => hash,
        }
    }

    /// Get object method
    pub fn method<S: AsRef<str>>(self, name: S) -> Option<Value> {
        if self.tag() != Tag::OBJECT {
            return None;
        }

        let v = unsafe { sys::caml_get_public_method(self.0, Self::hash_variant(name, None).0) };

        if v == 0 {
            return None;
        }

        Some(Value(v))
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated inside the OCaml heap,
    /// and may end up being moved or garbage collected.
    ///
    /// Requires the `deep-clone` feature
    #[cfg(feature = "deep-clone")]
    pub fn deep_clone_to_ocaml(self) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            let wosize = sys::wosize_val(self.0);
            let val1 = Self::alloc(wosize, self.tag());
            let ptr0 = self.ptr_val::<sys::Value>();
            let ptr1 = val1.mut_ptr_val::<sys::Value>();
            if self.tag() >= Tag::NO_SCAN {
                ptr0.copy_to_nonoverlapping(ptr1, wosize);
                return val1;
            }
            for i in 0..(wosize as isize) {
                sys::caml_initialize(
                    ptr1.offset(i),
                    Value(ptr0.offset(i).read()).deep_clone_to_ocaml().0,
                );
            }
            val1
        }
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated outside of the OCaml heap, and should
    /// only be used for storage inside Rust structures.
    ///
    /// Requires the `deep-clone` feature
    #[cfg(feature = "deep-clone")]
    pub fn deep_clone_to_rust(self) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            if self.tag() >= Tag::NO_SCAN {
                let slice0 = sys::as_slice(self.0);
                let vec1 = slice0.to_vec();
                let ptr1 = vec1.as_ptr();
                std::mem::forget(vec1);
                return Value::ptr(ptr1.offset(1));
            }
            let slice0 = sys::as_slice(self.0);
            let vec1: Vec<sys::Value> = slice0
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if i == 0 {
                        *v
                    } else {
                        Value(*v).deep_clone_to_rust().0
                    }
                })
                .collect();
            let ptr1 = vec1.as_ptr();
            mem::forget(vec1);
            Value::ptr(ptr1.offset(1))
        }
    }
}
