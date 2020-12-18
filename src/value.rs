use crate::error::{CamlError, Error};
use crate::tag::Tag;
use crate::{interop::ToOCaml, sys, OCaml, Runtime};

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = sys::Size;

/// Value wraps the native OCaml `value` type transparently, this means it has the
/// same representation as an `ocaml_sys::Value`
#[derive(Debug, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value(pub sys::Value);

impl Clone for Value {
    fn clone(&self) -> Value {
        unsafe { Value::new(self.0) }
    }
}

/// `ToValue` is used to convert from Rust types to OCaml values
pub unsafe trait ToValue {
    /// Convert to OCaml value
    fn to_value(self, rt: &mut Runtime) -> Value;
}

/// `FromValue` is used to convert from OCaml values to Rust types
pub unsafe trait FromValue {
    /// Convert from OCaml value
    fn from_value(v: Value) -> Self;
}

unsafe impl ToValue for Value {
    fn to_value(self, _rt: &mut Runtime) -> Value {
        unsafe { Value::new(self.0) }
    }
}

unsafe impl FromValue for Value {
    #[inline]
    fn from_value(v: Value) -> Value {
        v
    }
}

unsafe impl<'a, T> ToValue for OCaml<'a, T> {
    fn to_value(self, rt: &mut Runtime) -> Value {
        let mut frame = rt.open_frame();
        let gc = frame.initialize(&[]);
        let mut x = unsafe { crate::interop::internal::OCamlRoot::reserve(gc) };
        unsafe { Value::new(x.keep(self).get_raw()) }
    }
}

unsafe impl<'a, T> FromValue for OCaml<'a, T> {
    fn from_value(value: Value) -> OCaml<'a, T> {
        let rt = unsafe { &mut Runtime::recover_handle() };
        let x: OCaml<T> = unsafe { OCaml::new(rt, value.0) };
        unsafe { core::mem::transmute(x) }
    }
}

unsafe impl<'a> crate::interop::ToOCaml<Value> for Value {
    fn to_ocaml(
        &self,
        token: crate::interop::OCamlAllocToken,
    ) -> crate::interop::OCamlAllocResult<Value> {
        let mut gc = unsafe { &mut token.recover_runtime_handle() };
        unsafe { crate::interop::OCamlAllocResult::of_ocaml(OCaml::new(&mut gc, self.0)) }
    }
}

unsafe impl<'a> crate::interop::FromOCaml<Value> for Value {
    fn from_ocaml(v: OCaml<Value>) -> Value {
        unsafe { Value::new(v.raw()) }
    }
}

const NONE: Value = unsafe { Value::new(sys::NONE) };
const UNIT: Value = unsafe { Value::new(sys::UNIT) };

impl Value {
    /// Returns a named value registered by OCaml
    pub unsafe fn named<T: FromValue>(name: &str) -> Option<T> {
        let s = match crate::util::CString::new(name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let named = sys::caml_named_value(s.as_ptr());
        if named.is_null() {
            return None;
        }

        Some(FromValue::from_value(Value::new(*named)))
    }

    /// Allocate a new value with the given size and tag.
    pub unsafe fn alloc(rt: &mut Runtime, n: usize, tag: Tag) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_alloc(n, tag.into()));
            x
        })
    }

    /// Allocate a new tuple value
    pub unsafe fn alloc_tuple(rt: &mut Runtime, n: usize) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_alloc_tuple(n));
            x
        })
    }

    /// Allocate a new small value with the given size and tag
    pub unsafe fn alloc_small(rt: &mut Runtime, n: usize, tag: Tag) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_alloc_small(n, tag.into()));
            x
        })
    }

    /// Allocate a new value with a finalizer
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Pointer::alloc_custom` when possible.
    pub unsafe fn alloc_final<T>(
        rt: &mut Runtime,
        finalizer: unsafe extern "C" fn(Value),
        cfg: Option<(usize, usize)>,
    ) -> Value {
        let (used, max) = cfg.unwrap_or((0, 1));
        crate::frame!(rt, (x) {
            Value::new(sys::caml_alloc_final(
                core::mem::size_of::<T>(),
                core::mem::transmute(finalizer),
                used,
                max
            ))
        })
    }

    /// Allocate custom value
    pub unsafe fn alloc_custom<T: crate::Custom>(rt: &mut Runtime) -> Value {
        let size = core::mem::size_of::<T>();
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_alloc_custom(T::ops() as *const _ as *const sys::custom_operations, size, T::USED, T::MAX));
            x
        })
    }

    /// Allocate an abstract pointer value, it is best to ensure the value is
    /// on the heap using `Box::into_raw(Box::from(...))` to create the pointer
    /// and `Box::from_raw` to free it
    pub unsafe fn alloc_abstract_ptr<T>(rt: &mut Runtime, ptr: *mut T) -> Value {
        let x = Self::alloc(rt, 1, Tag::ABSTRACT);
        let dest = x.0 as *mut *mut T;
        *dest = ptr;
        x
    }

    /// Create a new Value from an existing OCaml `value`
    #[inline]
    pub const unsafe fn new(v: sys::Value) -> Value {
        Value(v)
    }

    /// Get array length
    pub unsafe fn array_length(self) -> usize {
        sys::caml_array_length(self.0)
    }

    /// See caml_register_global_root
    pub unsafe fn register_global_root(&mut self) {
        sys::caml_register_global_root(&mut self.0)
    }

    /// Set caml_remove_global_root
    pub unsafe fn remove_global_root(&mut self) {
        sys::caml_remove_global_root(&mut self.0)
    }

    /// Get the tag for the underlying OCaml `value`
    pub unsafe fn tag(self) -> Tag {
        sys::tag_val(self.0).into()
    }

    /// Convert a boolean to OCaml value
    pub const unsafe fn bool(b: bool) -> Value {
        Value::int(b as crate::Int)
    }

    /// Allocate and copy a string value
    pub unsafe fn string<S: AsRef<str>>(rt: &mut Runtime, s: S) -> Value {
        Value::new({
            let x: OCaml<ocaml_interop::OCamlBytes> = ocaml_interop::to_ocaml!(rt, s.as_ref());
            x.raw()
        })
    }

    /// Allocate and copy a byte array value
    pub unsafe fn bytes<S: AsRef<[u8]>>(rt: &mut Runtime, s: S) -> Value {
        Value::new({
            let x: OCaml<ocaml_interop::OCamlBytes> = ocaml_interop::to_ocaml!(rt, s.as_ref());
            x.raw()
        })
    }

    /// Convert from a pointer to an OCaml string back to an OCaml value
    ///
    /// # Safety
    /// This function assumes that the `str` argument has been allocated by OCaml
    pub unsafe fn of_str(s: &str) -> Value {
        Value::new(s.as_ptr() as isize)
    }

    /// Convert from a pointer to an OCaml string back to an OCaml value
    ///
    /// # Safety
    /// This function assumes that the `&[u8]` argument has been allocated by OCaml
    pub unsafe fn of_bytes(s: &[u8]) -> Value {
        Value::new(s.as_ptr() as isize)
    }

    /// OCaml Some value
    pub unsafe fn some<V: ToValue>(rt: &mut Runtime, v: V) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_alloc(1, 0));
            x.store_field(rt, 0, v);
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
    pub unsafe fn variant(rt: &mut Runtime, tag: u8, value: Option<Value>) -> Value {
        crate::frame!(rt, (x) {
            match value {
                Some(v) => {
                    x = Value::new(sys::caml_alloc(1, tag));
                    x.store_field(rt, 0, v)
                },
                None => x = Value::new(sys::caml_alloc(0, tag)),
            }
            x
        })
    }

    /// Result.Ok value
    pub unsafe fn result_ok(rt: &mut Runtime, value: impl Into<Value>) -> Value {
        Self::variant(rt, 0, Some(value.into()))
    }

    /// Result.Error value
    pub unsafe fn result_error(rt: &mut Runtime, value: impl Into<Value>) -> Value {
        Self::variant(rt, 1, Some(value.into()))
    }

    /// Create an OCaml `int`
    pub const unsafe fn int(i: crate::Int) -> Value {
        Value::new(sys::val_int(i))
    }

    /// Create an OCaml `int`
    pub const unsafe fn uint(i: crate::Uint) -> Value {
        Value::new(sys::val_int(i as crate::Int))
    }

    /// Create an OCaml `Int64` from `i64`
    pub unsafe fn int64(rt: &mut Runtime, i: i64) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_copy_int64(i));
            x
        })
    }

    /// Create an OCaml `Int32` from `i32`
    pub unsafe fn int32(rt: &mut Runtime, i: i32) -> Value {
        crate::frame!(rt, (x) {
            x = Value::new(sys::caml_copy_int32(i));
            x
        })
    }

    /// Create an OCaml `Nativeint` from `isize`
    pub unsafe fn nativeint(rt: &mut Runtime, i: isize) -> Value {
        frame!(rt, (x) {
            x = Value::new(sys::caml_copy_nativeint(i));
            x
        })
    }

    /// Create an OCaml `Float` from `f64`
    pub unsafe fn float(rt: &mut Runtime, d: f64) -> Value {
        frame!(rt, (x) {
            x = Value::new(sys::caml_copy_double(d));
            x
        })
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub unsafe fn is_block(self) -> bool {
        sys::is_block(self.0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub unsafe fn is_long(self) -> bool {
        sys::is_long(self.0)
    }

    /// Get index of underlying OCaml block value
    pub unsafe fn field<T: FromValue>(self, i: Size) -> T {
        T::from_value(Value::new(*sys::field(self.0, i)))
    }

    /// Set index of underlying OCaml block value
    pub unsafe fn store_field<V: ToValue>(&mut self, rt: &mut Runtime, i: Size, val: V) {
        sys::store_field(self.0, i, val.to_value(rt).0)
    }

    /// Convert an OCaml `int` to `isize`
    pub const unsafe fn int_val(self) -> isize {
        sys::int_val(self.0)
    }

    /// Convert an OCaml `Float` to `f64`
    pub unsafe fn float_val(self) -> f64 {
        *(self.0 as *const f64)
    }

    /// Convert an OCaml `Int32` to `i32`
    pub unsafe fn int32_val(self) -> i32 {
        *self.custom_ptr_val::<i32>()
    }

    /// Convert an OCaml `Int64` to `i64`
    pub unsafe fn int64_val(self) -> i64 {
        *self.custom_ptr_val::<i64>()
    }

    /// Convert an OCaml `Nativeint` to `isize`
    pub unsafe fn nativeint_val(self) -> isize {
        *self.custom_ptr_val::<isize>()
    }

    /// Get pointer to data stored in an OCaml custom value
    pub unsafe fn custom_ptr_val<T>(self) -> *const T {
        sys::field(self.0, 1) as *const T
    }

    /// Get mutable pointer to data stored in an OCaml custom value
    pub unsafe fn custom_ptr_val_mut<T>(self) -> *mut T {
        sys::field(self.0, 1) as *mut T
    }

    /// Get pointer to the pointer contained by Value
    pub unsafe fn abstract_ptr_val<T>(self) -> *const T {
        *(self.0 as *const *const T)
    }

    /// Get mutable pointer to the pointer contained by Value
    pub unsafe fn abstract_ptr_val_mut<T>(self) -> *mut T {
        *(self.0 as *mut *mut T)
    }

    /// Get underlying string pointer
    pub unsafe fn string_val(&self) -> &str {
        let len = crate::sys::caml_string_length(self.0);
        let ptr = crate::sys::string_val(self.0);
        let slice = ::core::slice::from_raw_parts(ptr, len);
        ::core::str::from_utf8(slice).expect("Invalid UTF-8").into()
    }

    /// Get underlying bytes pointer
    pub unsafe fn bytes_val(&self) -> &[u8] {
        let len = crate::sys::caml_string_length(self.0);
        let ptr = crate::sys::string_val(self.0);
        ::core::slice::from_raw_parts(ptr, len)
    }

    /// Get mutable string pointer
    pub unsafe fn string_val_mut<'a>(&'a mut self) -> &'a mut str {
        let len = crate::sys::caml_string_length(self.0);
        let ptr = crate::sys::string_val(self.0);

        let slice = ::core::slice::from_raw_parts_mut(ptr, len);
        ::core::str::from_utf8_mut(slice)
            .expect("Invalid UTF-8")
            .into()
    }

    /// Get mutable bytes pointer
    pub unsafe fn bytes_val_mut<'a>(&'a mut self) -> &'a mut [u8] {
        let len = crate::sys::caml_string_length(self.0);
        let ptr = crate::sys::string_val(self.0);
        ::core::slice::from_raw_parts_mut(ptr, len)
    }

    /// Extract OCaml exception
    pub unsafe fn exception<A: FromValue>(self) -> Option<A> {
        if !self.is_exception_result() {
            return None;
        }

        Some(A::from_value(Value::new(crate::sys::extract_exception(
            self.0,
        ))))
    }

    /// Call a closure with a single argument, returning an exception value
    pub unsafe fn call<A: ToValue>(self, rt: &mut Runtime, arg: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!(rt, (res) {
            res = Value::new(sys::caml_callback_exn(self.0, arg.to_value(rt).0));
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
    pub unsafe fn call2<A: ToValue, B: ToValue>(
        self,
        rt: &mut Runtime,
        arg1: A,
        arg2: B,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!(rt, (res) {
            res =
                Value::new(sys::caml_callback2_exn(self.0, arg1.to_value(rt).0, arg2.to_value(rt).0));
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
    pub unsafe fn call3<A: ToValue, B: ToValue, C: ToValue>(
        self,
        rt: &mut Runtime,
        arg1: A,
        arg2: B,
        arg3: C,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = crate::frame!(rt, (res) {
            res =
                Value::new(sys::caml_callback3_exn(
                    self.0,
                    arg1.to_value(rt).0,
                    arg2.to_value(rt).0,
                    arg3.to_value(rt).0,
                ));
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
    pub unsafe fn call_n<A: AsRef<[Value]>>(
        self,
        rt: &mut Runtime,
        args: A,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.as_ref().len();
        let x = args.as_ref();

        let mut v = crate::frame!(rt, (res) {
            res =
                Value::new(sys::caml_callbackN_exn(
                    self.0,
                    n,
                    x.as_ptr() as *mut sys::Value,
                ));
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
    pub unsafe fn modify<V: ToValue>(&mut self, rt: &mut Runtime, v: V) {
        sys::caml_modify(&mut self.0, v.to_value(rt).0)
    }

    /// Determines if the current value is an exception
    pub unsafe fn is_exception_result(self) -> bool {
        crate::sys::is_exception_result(self.0)
    }

    /// Get hash variant as OCaml value
    pub unsafe fn hash_variant<S: AsRef<str>>(
        rt: &mut Runtime,
        name: S,
        a: Option<Value>,
    ) -> Value {
        let s = crate::util::CString::new(name.as_ref()).expect("Invalid C string");
        let hash = Value::new(sys::caml_hash_variant(s.as_ptr() as *const u8));
        match a {
            Some(x) => {
                let mut output = Value::alloc_small(rt, 2, Tag(0));
                output.store_field(rt, 0, hash);
                output.store_field(rt, 1, x);
                output
            }
            None => hash,
        }
    }

    /// Get object method
    pub unsafe fn method<S: AsRef<str>>(self, rt: &mut Runtime, name: S) -> Option<Value> {
        if self.tag() != Tag::OBJECT {
            return None;
        }

        let v = sys::caml_get_public_method(self.0, Self::hash_variant(rt, name, None).0);

        if v == 0 {
            return None;
        }

        Some(Value::new(v))
    }

    /// Initialize OCaml value using `caml_initialize`
    pub unsafe fn initialize(&mut self, value: Value) {
        sys::caml_initialize(&mut self.0, value.0)
    }

    #[cfg(not(feature = "no-std"))]
    unsafe fn slice<'a>(self) -> &'a [Value] {
        ::core::slice::from_raw_parts(
            (self.0 as *const Value).offset(-1),
            sys::wosize_val(self.0) + 1,
        )
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated inside the OCaml heap,
    /// and may end up being moved or garbage collected.
    pub unsafe fn deep_clone_to_ocaml(self, rt: &mut Runtime) -> Self {
        if self.is_long() {
            return self;
        }
        let wosize = sys::wosize_val(self.0);
        let val1 = Self::alloc(rt, wosize, self.tag());
        let ptr0 = self.0 as *const sys::Value;
        let ptr1 = val1.0 as *mut sys::Value;
        if self.tag() >= Tag::NO_SCAN {
            ptr0.copy_to_nonoverlapping(ptr1, wosize);
            return val1;
        }
        for i in 0..(wosize as isize) {
            sys::caml_initialize(
                ptr1.offset(i),
                Value::new(ptr0.offset(i).read()).deep_clone_to_ocaml(rt).0,
            );
        }
        val1
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated outside of the OCaml heap, and should
    /// only be used for storage inside Rust structures.
    #[cfg(not(feature = "no-std"))]
    pub unsafe fn deep_clone_to_rust(self) -> Self {
        if self.is_long() {
            return self;
        }
        if self.tag() >= Tag::NO_SCAN {
            let slice0 = Value::slice(self);
            let vec1 = slice0.to_vec();
            let ptr1 = vec1.as_ptr();
            core::mem::forget(vec1);
            return Value::new(ptr1.offset(1) as isize);
        }
        let slice0 = Value::slice(self);
        let vec1: Vec<Value> = slice0
            .iter()
            .enumerate()
            .map(|(i, v)| if i == 0 { *v } else { v.deep_clone_to_rust() })
            .collect();
        let ptr1 = vec1.as_ptr();
        core::mem::forget(vec1);
        Value::new(ptr1.offset(1) as isize)
    }
}
