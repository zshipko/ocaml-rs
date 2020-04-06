use crate::error::Error;
use crate::runtime::hash_variant;
use crate::sys;
use crate::sys::caml_frame;
use crate::{tag, Tag};

use std::{mem, ptr};

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = sys::mlvalues::Size;

/// Value wraps the native OCaml `value` type transparently, this means it has the
/// same representation as an `ocaml_sys::mlvalues::Value`
#[derive(Debug, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value(pub sys::mlvalues::Value);

impl Clone for Value {
    fn clone(&self) -> Value {
        Value::new(self.0)
    }
}

/// `ToValue` is used to convert from Rust types to OCaml values
pub trait ToValue {
    /// Convert to OCaml value
    fn to_value(&self) -> Value;
}

/// `FromValue` is used to convert from OCaml values to Rust types
pub trait FromValue {
    /// Convert from OCaml value
    fn from_value(v: Value) -> Self;
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        Value(self.0)
    }
}

impl<'a> ToValue for &'a Value {
    fn to_value(&self) -> Value {
        Value(self.0)
    }
}

impl FromValue for Value {
    #[inline]
    fn from_value(v: Value) -> Value {
        v
    }
}

const TRUE: Value = Value(sys::mlvalues::val_int(1));
const FALSE: Value = Value(sys::mlvalues::val_int(0));
const NONE: Value = Value(sys::mlvalues::val_int(0));
const UNIT: Value = Value(sys::mlvalues::UNIT);

impl Value {
    /// Allocate a new value with the given size and tag.
    pub fn alloc(n: usize, tag: Tag) -> Value {
        Value(sys::caml_frame!(|x| {
            x = unsafe { sys::alloc::caml_alloc(n, tag) };
            x
        }))
    }

    /// Allocate a new tuple value
    pub fn alloc_tuple(n: usize) -> Value {
        Value(sys::caml_frame!(|x| {
            x = unsafe { sys::alloc::caml_alloc_tuple(n) };
            x
        }))
    }

    /// Allocate a new small value with the given size and tag
    pub fn alloc_small(n: usize, tag: Tag) -> Value {
        Value(sys::caml_frame!(|x| {
            x = unsafe { sys::alloc::caml_alloc_small(n, tag) };
            x
        }))
    }

    /// Allocate a new value with a custom finalizer
    /// NOTE: `value` will be copied into memory allocated by the OCaml runtime, you are
    /// responsible for managing the lifetime of the value on the Rust side
    pub fn alloc_custom<T>(value: *const T, finalizer: extern "C" fn(Value)) -> Value {
        Value(sys::caml_frame!(|x| {
            x = unsafe {
                sys::alloc::caml_alloc_final(
                    std::mem::size_of::<T>(),
                    std::mem::transmute(finalizer),
                    0,
                    1,
                )
            };

            let ptr = Value(x).custom_ptr_val_mut::<T>();
            unsafe { std::ptr::copy(value, ptr, 1) };
            x
        }))
    }

    /// Set custom pointer value
    pub fn set_custom<T>(&mut self, value: T) -> T {
        let ptr = self.custom_ptr_val_mut::<T>();
        unsafe { ptr::replace(ptr, value) }
    }

    /// Create a new Value from an existing OCaml `value`
    #[inline]
    pub const fn new(v: sys::mlvalues::Value) -> Value {
        Value(v)
    }

    /// Get array length
    pub fn array_length(self) -> usize {
        unsafe { sys::mlvalues::caml_array_length(self.0) }
    }

    /// See caml_register_global_root
    pub fn register_global_root(self) {
        unsafe { sys::memory::caml_register_global_root(&self.0) }
    }

    /// Set caml_remove_global_root
    pub fn remove_global_root(self) {
        unsafe { sys::memory::caml_remove_global_root(&self.0) }
    }

    /// Get the tag for the underlying OCaml `value`
    pub fn tag(self) -> Tag {
        unsafe { sys::mlvalues::tag_val(self.0) }
    }

    /// Convert a boolean to OCaml value
    pub fn bool(b: bool) -> Value {
        if b {
            TRUE
        } else {
            FALSE
        }
    }

    /// OCaml Some value
    pub fn some<V: ToValue>(v: V) -> Value {
        Value(caml_frame!(|x| {
            unsafe {
                x = sys::alloc::caml_alloc(1, 0);
                sys::memory::store_field(x, 0, v.to_value().0);
            }
            x
        }))
    }

    /// OCaml None value
    pub const NONE: Value = NONE;

    /// OCaml Unit value
    pub const UNIT: Value = UNIT;

    /// Create a variant value
    pub fn variant<V: ToValue>(tag: u8, value: Option<V>) -> Value {
        Value(caml_frame!(|x| {
            match value {
                Some(v) => unsafe {
                    x = sys::alloc::caml_alloc(1, tag);
                    sys::memory::store_field(x, 0, v.to_value().0)
                },
                None => x = unsafe { sys::alloc::caml_alloc(0, tag) },
            }
            x
        }))
    }

    /// Create a new opaque pointer Value
    pub fn ptr<T>(p: *const T) -> Value {
        Value(p as sys::mlvalues::Value)
    }

    /// Create an OCaml `int`
    pub fn int(i: isize) -> Value {
        Value(sys::mlvalues::val_int(i))
    }

    /// Create an OCaml `Int64` from `i64`
    pub fn int64(i: i64) -> Value {
        Value(caml_frame!(|x| {
            unsafe { x = sys::alloc::caml_copy_int64(i) };
            x
        }))
    }

    /// Create an OCaml `Int32` from `i32`
    pub fn int32(i: i32) -> Value {
        Value(caml_frame!(|x| {
            unsafe { x = sys::alloc::caml_copy_int32(i) };
            x
        }))
    }

    /// Create an OCaml `Nativeint` from `isize`
    pub fn nativeint(i: isize) -> Value {
        Value(caml_frame!(|x| {
            unsafe { x = sys::alloc::caml_copy_nativeint(i) };
            x
        }))
    }

    /// Create an OCaml `Float` from `f64`
    pub fn f64(d: f64) -> Value {
        Value(caml_frame!(|x| {
            unsafe { x = sys::alloc::caml_copy_double(d) }
            x
        }))
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub fn is_block(self) -> bool {
        sys::mlvalues::is_block(self.0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub fn is_long(self) -> bool {
        sys::mlvalues::is_long(self.0)
    }

    /// Get index of underlying OCaml block value
    pub fn field<T: FromValue>(self, i: Size) -> T {
        unsafe { T::from_value(Value(*sys::mlvalues::field(self.0, i))) }
    }

    /// Set index of underlying OCaml block value
    pub fn store_field<V: ToValue>(&mut self, i: Size, val: V) {
        unsafe { sys::memory::store_field(self.0, i, val.to_value().0) }
    }

    /// Convert an OCaml `int` to `isize`
    pub fn int_val(self) -> isize {
        sys::mlvalues::int_val(self.0)
    }

    /// Convert an OCaml `Float` to `f64`
    pub fn f64_val(self) -> f64 {
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
        unsafe { sys::mlvalues::field(self.0, 1) as *const T }
    }

    /// Get mutable pointer to data stored in an OCaml custom value
    pub fn custom_ptr_val_mut<T>(self) -> *mut T {
        unsafe { sys::mlvalues::field(self.0, 1) as *mut T }
    }

    /// Get pointer to data stored in an opaque value
    pub fn ptr_val<T>(self) -> *const T {
        self.0 as *const T
    }

    /// Get mutable pointer to data stored in an opaque value
    pub fn mut_ptr_val<T>(self) -> *mut T {
        self.0 as *mut T
    }

    /// Call a closure with a single argument, returning an exception value
    pub fn call<A: ToValue>(self, arg: A) -> Result<Value, Error> {
        if self.tag() != tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = caml_frame!(|res| {
            res = unsafe { sys::callback::caml_callback_exn(self.0, arg.to_value().0) };
            res
        });

        if crate::sys::callback::is_exception_result(v) {
            v = crate::sys::callback::extract_exception(v);
            Err(Error::Exception(Value(v)))
        } else {
            Ok(Value(v))
        }
    }

    /// Call a closure with two arguments, returning an exception value
    pub fn call2<A: ToValue, B: ToValue>(self, arg1: A, arg2: B) -> Result<Value, Error> {
        if self.tag() != tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = caml_frame!(|res| {
            res = unsafe {
                sys::callback::caml_callback2_exn(self.0, arg1.to_value().0, arg2.to_value().0)
            };
            res
        });

        if crate::sys::callback::is_exception_result(v) {
            v = crate::sys::callback::extract_exception(v);
            Err(Error::Exception(Value(v)))
        } else {
            Ok(Value(v))
        }
    }

    /// Call a closure with three arguments, returning an exception value
    pub fn call3<A: ToValue, B: ToValue, C: ToValue>(
        self,
        arg1: A,
        arg2: B,
        arg3: C,
    ) -> Result<Value, Error> {
        if self.tag() != tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = caml_frame!(|res| {
            res = unsafe {
                sys::callback::caml_callback3_exn(
                    self.0,
                    arg1.to_value().0,
                    arg2.to_value().0,
                    arg3.to_value().0,
                )
            };
            res
        });

        if crate::sys::callback::is_exception_result(v) {
            v = crate::sys::callback::extract_exception(v);
            Err(Error::Exception(Value(v)))
        } else {
            Ok(Value(v))
        }
    }

    /// Call a closure with `n` arguments, returning an exception value
    pub fn call_n<A: AsRef<[Value]>>(self, args: A) -> Result<Value, Error> {
        if self.tag() != tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.as_ref().len();
        let x: Vec<sys::mlvalues::Value> = args.as_ref().iter().map(|x| x.0).collect();

        let mut v = caml_frame!(|res| {
            res = unsafe {
                sys::callback::caml_callbackN_exn(
                    self.0,
                    n,
                    x.as_ptr() as *mut sys::mlvalues::Value,
                )
            };
            res
        });

        if crate::sys::callback::is_exception_result(v) {
            v = crate::sys::callback::extract_exception(v);
            Err(Error::Exception(Value(v)))
        } else {
            Ok(Value(v))
        }
    }

    /// Modify an OCaml value in place
    pub fn modify<V: ToValue>(&mut self, v: V) {
        unsafe { sys::memory::caml_modify(&mut self.0, v.to_value().0) }
    }

    /// Determines if the current value is an exception
    pub fn is_exception_result(self) -> bool {
        crate::sys::callback::is_exception_result(self.0)
    }

    /// Get object method
    pub fn method<S: AsRef<str>>(self, name: S) -> Option<Value> {
        if self.tag() != tag::OBJECT {
            return None;
        }

        let v = unsafe { sys::mlvalues::caml_get_public_method(self.0, hash_variant(name).0) };

        if v == 0 {
            return None;
        }

        Some(Value::new(v))
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated inside the OCaml heap,
    /// and may end up being moved or garbage collected.
    pub fn deep_clone_to_ocaml(self) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            let wosize = sys::mlvalues::wosize_val(self.0);
            let val1 = Self::alloc(wosize, self.tag());
            let ptr0 = self.ptr_val::<sys::mlvalues::Value>();
            let ptr1 = val1.mut_ptr_val::<sys::mlvalues::Value>();
            if sys::mlvalues::tag_val(self.0) >= tag::NO_SCAN {
                ptr0.copy_to_nonoverlapping(ptr1, wosize);
                return val1;
            }
            for i in 0..(wosize as isize) {
                sys::memory::caml_initialize(
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
    pub fn deep_clone_to_rust(self) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            if sys::mlvalues::tag_val(self.0) >= tag::NO_SCAN {
                let slice0 = sys::mlvalues::as_slice(self.0);
                let vec1 = slice0.to_vec();
                let ptr1 = vec1.as_ptr();
                mem::forget(vec1);
                return Value::ptr(ptr1.offset(1));
            }
            let slice0 = sys::mlvalues::as_slice(self.0);
            let vec1: Vec<sys::mlvalues::Value> = slice0
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
