use crate::error::{CamlError, Error};
use crate::tag::Tag;
use crate::{interop::BoxRoot, root::Root, sys, util, OCaml, OCamlRef, Pointer, Runtime};

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = sys::Size;

/// Value wraps the native OCaml `value` type
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Value {
    /// Rooted value
    Root(Root),

    /// Reference to a rooted value
    /// NOTE: `Value::Raw` should NOT be used to convert arbitrary `sys::Value` into `Value`
    Raw(sys::Value),
}

/// Wrapper around sys::Value
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
pub struct Raw(pub sys::Value);

impl Raw {
    /// Convert a `Raw` value to `Value`, this should only be used in custom value destructors
    /// and other cases where you know the underlying `sys::Value` was created using `Value::new`
    /// or it is not possible for the value to be garbage collected (i.e. inside of a `@@noalloc`
    /// function or in a custom type finalizer)
    pub unsafe fn as_value(&self) -> Value {
        Value::Raw(self.0)
    }

    /// Convenience function to access a custom pointer value
    pub unsafe fn as_pointer<T>(&self) -> Pointer<T> {
        Pointer::from_value(self.as_value())
    }
}

impl AsRef<sys::Value> for Raw {
    fn as_ref(&self) -> &sys::Value {
        &self.0
    }
}

impl From<sys::Value> for Raw {
    fn from(x: sys::Value) -> Raw {
        Raw(x)
    }
}

impl From<Raw> for sys::Value {
    fn from(x: Raw) -> sys::Value {
        x.0
    }
}

impl From<sys::Value> for Value {
    fn from(v: sys::Value) -> Self {
        unsafe { Value::Root(Root::new(v)) }
    }
}

impl From<Raw> for Value {
    fn from(v: Raw) -> Self {
        Value::Raw(v.into())
    }
}

/// `ToValue` is used to convert from Rust types to OCaml values
///
/// NOTE: This should only be used after the OCaml runtime has been initialized, when calling
/// Rust functions from OCaml, the runtime is already initialized otherwise `ocaml::Runtime::init`
/// should be used
pub unsafe trait ToValue {
    /// Convert to OCaml value
    fn to_value(&self, rt: &Runtime) -> Value;
}

/// `FromValue` is used to convert from OCaml values to Rust types
///
/// NOTE: This should only be used after the OCaml runtime has been initialized, when calling
/// Rust functions from OCaml, the runtime is already initialized otherwise `ocaml::Runtime::init`
/// should be used
pub unsafe trait FromValue {
    /// Convert from OCaml value
    fn from_value(v: Value) -> Self;
}

unsafe impl ToValue for Value {
    fn to_value(&self, _rt: &Runtime) -> Value {
        self.clone()
    }
}

unsafe impl FromValue for Value {
    fn from_value(v: Value) -> Value {
        v
    }
}

unsafe impl ToValue for Raw {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::new(self.0) }
    }
}

unsafe impl FromValue for Raw {
    #[inline]
    fn from_value(v: Value) -> Raw {
        v.raw()
    }
}

unsafe impl<'a, T> ToValue for OCaml<'a, T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::new(self.raw()) }
    }
}

unsafe impl<'a, T> ToValue for OCamlRef<'a, T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::new(self.get_raw()) }
    }
}

unsafe impl<T> ToValue for BoxRoot<T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::new(self.get_raw()) }
    }
}

unsafe impl<T> FromValue for BoxRoot<T> {
    fn from_value(v: Value) -> BoxRoot<T> {
        let ocaml: OCaml<T> = FromValue::from_value(v);
        ocaml.root()
    }
}

unsafe impl<'a, T> FromValue for OCaml<'a, T> {
    fn from_value<'b>(v: Value) -> OCaml<'a, T> {
        // NOTE: this should only be used after the runtime is initialized
        let rt = unsafe { Runtime::recover_handle() };
        unsafe { OCaml::new(rt, v.raw().into()) }
    }
}

unsafe impl<T> crate::interop::ToOCaml<T> for Value {
    fn to_ocaml<'a>(&self, gc: &'a mut Runtime) -> OCaml<'a, T> {
        unsafe { OCaml::new(gc, self.raw().into()) }
    }
}

unsafe impl<T> crate::interop::FromOCaml<T> for Value {
    fn from_ocaml(v: OCaml<T>) -> Value {
        unsafe { Value::new(v.raw()) }
    }
}

impl Value {
    /// Get raw OCaml value
    pub fn raw(&self) -> Raw {
        match self {
            Value::Root(r) => unsafe { r.get().into() },
            Value::Raw(r) => Raw(*r),
        }
    }

    /// Helper function to convert from `Value` to `T`
    pub fn to<T: FromValue>(&self) -> T {
        T::from_value(Value::Raw(self.raw().0))
    }

    /// Helper function to convert from `Value` to `T`
    pub fn into<T: FromValue>(self) -> T {
        T::from_value(self)
    }

    /// Returns a named value registered by OCaml
    pub unsafe fn named(name: &str) -> Option<Value> {
        let s = match util::CString::new(name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let named = sys::caml_named_value(s.as_ptr());
        if named.is_null() {
            return None;
        }

        Some(Value::new(*named))
    }

    /// Allocate a new value with the given size and tag.
    pub unsafe fn alloc(n: usize, tag: Tag) -> Value {
        Value::new(sys::caml_alloc(n, tag.into()))
    }

    /// Allocate a new float array
    pub unsafe fn alloc_double_array(n: usize) -> Value {
        Value::new(sys::caml_alloc_float_array(n))
    }

    /// Allocate a new tuple value
    pub unsafe fn alloc_tuple(n: usize) -> Value {
        Value::new(sys::caml_alloc_tuple(n))
    }

    /// Allocate a new small value with the given size and tag
    pub unsafe fn alloc_small(n: usize, tag: Tag) -> Value {
        Value::new(sys::caml_alloc_small(n, tag.into()))
    }

    /// Allocate a new value with a finalizer
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Pointer::alloc_custom` when possible.
    pub unsafe fn alloc_final<T>(
        finalizer: unsafe extern "C" fn(Raw),
        cfg: Option<(usize, usize)>,
    ) -> Value {
        let (used, max) = cfg.unwrap_or((0, 1));
        Value::new(sys::caml_alloc_final(
            core::mem::size_of::<T>(),
            core::mem::transmute(finalizer),
            used,
            max,
        ))
    }

    /// Allocate custom value
    pub unsafe fn alloc_custom<T: crate::Custom>() -> Value {
        let size = core::mem::size_of::<T>();
        Value::new(sys::caml_alloc_custom(
            T::ops() as *const _ as *const sys::custom_operations,
            size,
            T::USED,
            T::MAX,
        ))
    }

    /// Allocate an abstract pointer value, it is best to ensure the value is
    /// on the heap using `Box::into_raw(Box::from(...))` to create the pointer
    /// and `Box::from_raw` to free it
    pub unsafe fn alloc_abstract_ptr<T>(ptr: *mut T) -> Value {
        let x = Self::alloc(1, Tag::ABSTRACT);
        let dest = x.raw().0 as *mut *mut T;
        *dest = ptr;
        x
    }

    #[inline]
    /// Create a new Value from an existing OCaml `value`
    pub unsafe fn new<T: Into<Value>>(v: T) -> Value {
        v.into()
    }

    /// Get array length
    pub unsafe fn array_length(&self) -> usize {
        sys::caml_array_length(self.raw().into())
    }

    /// See caml_register_global_root
    pub unsafe fn register_global_root(&mut self) {
        sys::caml_register_global_root(&mut self.raw().0)
    }

    /// Set caml_remove_global_root
    pub unsafe fn remove_global_root(&mut self) {
        sys::caml_remove_global_root(&mut self.raw().0)
    }

    /// Get the tag for the underlying OCaml `value`
    pub unsafe fn tag(&self) -> Tag {
        sys::tag_val(self.raw().0).into()
    }

    /// Convert a boolean to OCaml value
    pub unsafe fn bool(b: bool) -> Value {
        Value::int(b as crate::Int)
    }

    /// Allocate and copy a string value
    pub unsafe fn string<S: AsRef<str>>(s: S) -> Value {
        let s = s.as_ref();
        Value::new(sys::caml_alloc_initialized_string(
            s.len(),
            s.as_ptr() as *const _,
        ))
    }

    /// Allocate and copy a byte array value
    pub unsafe fn bytes<S: AsRef<[u8]>>(s: S) -> Value {
        let s = s.as_ref();
        Value::new(sys::caml_alloc_initialized_string(
            s.len(),
            s.as_ptr() as *const _,
        ))
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
    pub unsafe fn some<V: ToValue>(rt: &Runtime, v: V) -> Value {
        let v = v.to_value(rt);
        let mut x = Value::new(sys::caml_alloc(1, 0));
        x.store_field(rt, 0, &v);
        x
    }

    /// OCaml None value
    #[inline(always)]
    pub fn none() -> Value {
        unsafe { Value::new(sys::NONE) }
    }

    /// OCaml Unit value
    #[inline(always)]
    pub fn unit() -> Value {
        unsafe { Value::new(sys::UNIT) }
    }

    /// Create a variant value
    pub unsafe fn variant(rt: &Runtime, tag: u8, value: Option<Value>) -> Value {
        match value {
            Some(v) => {
                let mut value = Value::new(sys::caml_alloc(1, tag));
                value.store_field(rt, 0, v);
                value
            }
            None => Value::new(sys::caml_alloc(0, tag)),
        }
    }

    /// Result.Ok value
    pub unsafe fn result_ok<T: ToValue>(rt: &Runtime, value: T) -> Value {
        Self::variant(rt, 0, Some(value.to_value(rt)))
    }

    /// Convert OCaml `('a, 'b) Result.t` to Rust `Result<Value, Value>`
    pub unsafe fn result<A: FromValue, B: FromValue>(&self) -> Result<A, B> {
        let tag = self.tag();
        if tag.0 == 0 {
            Ok(FromValue::from_value(self.field(0)))
        } else {
            Err(FromValue::from_value(self.field(0)))
        }
    }

    /// Result.Error value
    pub unsafe fn result_error<T: ToValue>(rt: &Runtime, value: T) -> Value {
        Self::variant(rt, 1, Some(value.to_value(rt)))
    }

    /// Create an OCaml `int`
    pub unsafe fn int(i: crate::Int) -> Value {
        Value::new(sys::val_int(i))
    }

    /// Create an OCaml `int`
    pub unsafe fn uint(i: crate::Uint) -> Value {
        Value::new(sys::val_int(i as crate::Int))
    }

    /// Create an OCaml `Int64` from `i64`
    pub unsafe fn int64(i: i64) -> Value {
        Value::new(sys::caml_copy_int64(i))
    }

    /// Create an OCaml `Int32` from `i32`
    pub unsafe fn int32(i: i32) -> Value {
        Value::new(sys::caml_copy_int32(i))
    }

    /// Create an OCaml `Nativeint` from `isize`
    pub unsafe fn nativeint(i: isize) -> Value {
        Value::new(sys::caml_copy_nativeint(i))
    }

    /// Create an OCaml `Float` from `f64`
    pub unsafe fn double(d: f64) -> Value {
        Value::new(sys::caml_copy_double(d))
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub unsafe fn is_block(&self) -> bool {
        sys::is_block(self.raw().0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub unsafe fn is_long(&self) -> bool {
        sys::is_long(self.raw().0)
    }

    /// Get index of underlying OCaml block value
    pub unsafe fn field(&self, i: Size) -> Value {
        Value::new(*sys::field(self.raw().0, i))
    }

    /// Get index of underlying OCaml double array value
    pub unsafe fn double_field(&self, i: Size) -> f64 {
        sys::caml_sys_double_field(self.raw().0, i)
    }

    /// Set index of underlying OCaml block value
    pub unsafe fn store_field<V: ToValue>(&mut self, rt: &Runtime, i: Size, val: V) {
        let v = val.to_value(rt);
        sys::store_field(self.raw().0, i, v.raw().0)
    }

    /// Set index of underlying OCaml double array value
    pub unsafe fn store_double_field(&mut self, i: Size, val: f64) {
        sys::caml_sys_store_double_field(self.raw().0, i, val)
    }

    /// Convert an OCaml `int` to `isize`
    pub unsafe fn int_val(&self) -> isize {
        sys::int_val(self.raw().0)
    }

    /// Convert an OCaml `Float` to `f64`
    pub unsafe fn double_val(&self) -> f64 {
        sys::caml_sys_double_val(self.raw().0)
    }

    /// Store `f64` in OCaml `Float`
    pub unsafe fn store_double_val(&mut self, val: f64) {
        sys::caml_sys_store_double_val(self.raw().0, val)
    }

    /// Convert an OCaml `Int32` to `i32`
    pub unsafe fn int32_val(&self) -> i32 {
        *self.custom_ptr_val::<i32>()
    }

    /// Convert an OCaml `Int64` to `i64`
    pub unsafe fn int64_val(&self) -> i64 {
        *self.custom_ptr_val::<i64>()
    }

    /// Convert an OCaml `Nativeint` to `isize`
    pub unsafe fn nativeint_val(&self) -> isize {
        *self.custom_ptr_val::<isize>()
    }

    /// Get pointer to data stored in an OCaml custom value
    pub unsafe fn custom_ptr_val<T>(&self) -> *const T {
        sys::field(self.raw().0, 1) as *const T
    }

    /// Get mutable pointer to data stored in an OCaml custom value
    pub unsafe fn custom_ptr_val_mut<T>(&mut self) -> *mut T {
        sys::field(self.raw().0, 1) as *mut T
    }

    /// Get pointer to the pointer contained by Value
    pub unsafe fn abstract_ptr_val<T>(&self) -> *const T {
        *(self.raw().0 as *const *const T)
    }

    /// Get mutable pointer to the pointer contained by Value
    pub unsafe fn abstract_ptr_val_mut<T>(&self) -> *mut T {
        *(self.raw().0 as *mut *mut T)
    }

    /// Get underlying string pointer
    pub unsafe fn string_val(&self) -> &str {
        let len = sys::caml_string_length(self.raw().0);
        let ptr = sys::string_val(self.raw().0);
        let slice = ::core::slice::from_raw_parts(ptr, len);
        ::core::str::from_utf8(slice).expect("Invalid UTF-8")
    }

    /// Get underlying bytes pointer
    pub unsafe fn bytes_val(&self) -> &[u8] {
        let len = sys::caml_string_length(self.raw().0);
        let ptr = sys::string_val(self.raw().0);
        ::core::slice::from_raw_parts(ptr, len)
    }

    /// Get mutable string pointer
    pub unsafe fn string_val_mut(&mut self) -> &mut str {
        let len = sys::caml_string_length(self.raw().0);
        let ptr = sys::string_val(self.raw().0);

        let slice = ::core::slice::from_raw_parts_mut(ptr, len);
        ::core::str::from_utf8_mut(slice).expect("Invalid UTF-8")
    }

    /// Get mutable bytes pointer
    pub unsafe fn bytes_val_mut(&mut self) -> &mut [u8] {
        let len = sys::caml_string_length(self.raw().0);
        let ptr = sys::string_val(self.raw().0);
        ::core::slice::from_raw_parts_mut(ptr, len)
    }

    /// Extract OCaml exception
    pub unsafe fn exception(&self) -> Option<Value> {
        if !self.is_exception_result() {
            return None;
        }

        Some(Value::new(sys::extract_exception(self.raw().0)))
    }

    /// Convert value to Rust Result type depending on if the value is an exception or not
    unsafe fn check_result(mut self) -> Result<Value, Error> {
        if !self.is_exception_result() {
            return Ok(self);
        }

        match &mut self {
            Value::Root(r) => {
                r.modify(sys::extract_exception(r.get()));
            }
            Value::Raw(mut r) => sys::caml_modify(&mut r, sys::extract_exception(r)),
        }

        Err(CamlError::Exception(self).into())
    }

    /// Call a closure with a single argument, returning an exception result
    pub unsafe fn call1<A: ToValue>(&self, rt: &Runtime, arg1: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let v = {
            let arg1 = arg1.to_value(rt);
            Value::new(sys::caml_callback_exn(self.raw().0, arg1.raw().0))
        };
        v.check_result()
    }

    /// Call a closure with two arguments, returning an exception result
    pub unsafe fn call2<A: ToValue, B: ToValue>(
        &self,
        rt: &Runtime,
        arg1: A,
        arg2: B,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let v = {
            let arg1 = arg1.to_value(rt);
            let arg2 = arg2.to_value(rt);

            Value::new(sys::caml_callback2_exn(
                self.raw().0,
                arg1.raw().0,
                arg2.raw().0,
            ))
        };

        v.check_result().map(Value::into)
    }

    /// Call a closure with three arguments, returning an exception result
    pub unsafe fn call3<A: ToValue, B: ToValue, C: ToValue>(
        &self,
        rt: &Runtime,
        arg1: A,
        arg2: B,
        arg3: C,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let v = {
            let arg1 = arg1.to_value(rt);
            let arg2 = arg2.to_value(rt);
            let arg3 = arg3.to_value(rt);

            Value::new(sys::caml_callback3_exn(
                self.raw().0,
                arg1.raw().0,
                arg2.raw().0,
                arg3.raw().0,
            ))
        };

        v.check_result().map(Value::into)
    }

    /// Call a closure with `n` arguments, returning an exception result
    pub unsafe fn call_n<A: AsRef<[Raw]>>(&self, args: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.as_ref().len();

        let v: Value = Value::new(sys::caml_callbackN_exn(
            self.raw().0,
            n,
            args.as_ref().as_ptr() as *mut sys::Value,
        ));

        v.check_result()
    }

    /// Call a closure with a variable number of arguments, returning and exception Result
    #[cfg(not(feature = "no-std"))]
    pub unsafe fn call<const N: usize, T: FromValue>(
        &self,
        rt: &Runtime,
        args: [impl ToValue; N],
    ) -> Result<T, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.len();
        let mut a = vec![];

        for arg in args {
            a.push(arg.to_value(rt));
        }

        if a.is_empty() {
            a.push(Value::unit());
        }

        let b: Vec<Raw> = a.iter().map(|x| x.raw()).collect();

        let v: Value = Value::new(sys::caml_callbackN_exn(
            self.raw().0,
            n,
            b.as_ptr() as *mut sys::Value,
        ));

        FromValue::from_value(v.check_result()?)
    }

    /// Modify an OCaml value in place
    pub unsafe fn modify<V: ToValue>(&mut self, rt: &Runtime, v: V) {
        let v = v.to_value(rt);
        match self {
            Value::Root(r) => {
                r.modify(v.raw().into());
            }
            Value::Raw(r) => sys::caml_modify(r, v.raw().into()),
        }
    }

    /// Modify an OCaml value in place using a raw OCaml value as the new value
    pub unsafe fn modify_raw(&mut self, v: Raw) {
        match self {
            Value::Root(r) => r.modify(v.into()),
            Value::Raw(r) => sys::caml_modify(r, v.into()),
        }
    }

    /// Determines if the current value is an exception
    pub unsafe fn is_exception_result(&self) -> bool {
        sys::is_exception_result(self.raw().0)
    }

    /// Get hash variant as OCaml value
    pub unsafe fn hash_variant<S: AsRef<str>>(rt: &Runtime, name: S, a: Option<Value>) -> Value {
        let s = util::CString::new(name.as_ref()).expect("Invalid C string");
        let hash = Value::new(sys::caml_hash_variant(s.as_ptr() as *const u8));
        match a {
            Some(x) => {
                let mut output = Value::alloc_small(2, Tag(0));
                output.store_field(rt, 0, &hash);
                output.store_field(rt, 1, &x);
                output
            }
            None => hash,
        }
    }

    /// Get object method
    pub unsafe fn method<S: AsRef<str>>(&self, rt: &Runtime, name: S) -> Option<Value> {
        if self.tag() != Tag::OBJECT {
            return None;
        }

        let variant = Self::hash_variant(rt, name, None);
        let v = sys::caml_get_public_method(self.raw().0, variant.raw().0);

        if v == 0 {
            return None;
        }

        Some(Value::new(v))
    }

    /// Returns the next item and the next `Seq.t` if available, otherwise `Ok(None)`
    pub unsafe fn seq_next(&self) -> Result<Option<(Value, Value)>, Error> {
        let x = self.call_n([Value::unit().raw()])?;

        if !x.is_block() {
            return Ok(None);
        }

        let v = x.field(0);
        let next = x.field(1);

        Ok(Some((v, next)))
    }

    /// Convert an OCaml exception value to the string representation
    #[cfg(not(feature = "no-std"))]
    pub unsafe fn exception_to_string(&self) -> Result<String, core::str::Utf8Error> {
        let ptr = ocaml_sys::caml_format_exception(self.raw().0);
        std::ffi::CStr::from_ptr(ptr).to_str().map(|x| x.to_owned())
    }

    /// Initialize OCaml value using `caml_initialize`
    pub unsafe fn initialize(&mut self, value: Value) {
        sys::caml_initialize(&mut self.raw().0, value.raw().0)
    }

    #[doc(hidden)]
    pub unsafe fn slice<'a>(&self) -> &'a [Raw] {
        ::core::slice::from_raw_parts(
            (self.raw().0 as *const Raw).offset(-1),
            sys::wosize_val(self.raw().0) + 1,
        )
    }

    #[doc(hidden)]
    pub unsafe fn slice_mut<'a>(&self) -> &'a mut [Raw] {
        ::core::slice::from_raw_parts_mut(
            (self.raw().0 as *mut Raw).offset(-1),
            sys::wosize_val(self.raw().0) + 1,
        )
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated inside the OCaml heap,
    /// and may end up being moved or garbage collected.
    pub unsafe fn deep_clone_to_ocaml(self) -> Self {
        if self.is_long() {
            return self;
        }
        let wosize = sys::wosize_val(self.raw().0);
        let val1 = Self::alloc(wosize, self.tag());
        let ptr0 = self.raw().0 as *const sys::Value;
        let ptr1 = val1.raw().0 as *mut sys::Value;
        if self.tag() >= Tag::NO_SCAN {
            ptr0.copy_to_nonoverlapping(ptr1, wosize);
            return val1;
        }
        for i in 0..(wosize as isize) {
            sys::caml_initialize(
                ptr1.offset(i),
                Value::new(ptr0.offset(i).read())
                    .deep_clone_to_ocaml()
                    .raw()
                    .0,
            );
        }
        val1
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated outside of the OCaml heap, and should
    /// only be used for storage inside Rust structures.
    #[cfg(not(feature = "no-std"))]
    pub unsafe fn deep_clone_to_rust(&self) -> Self {
        if self.is_long() {
            return self.clone();
        }
        if self.tag() >= Tag::NO_SCAN {
            let slice0 = self.slice();
            let vec1 = slice0.to_vec();
            let ptr1 = vec1.as_ptr();
            core::mem::forget(vec1);
            return Value::new(ptr1.offset(1) as isize);
        }
        let slice0 = self.slice();
        Value::new(slice0.as_ptr().offset(1) as isize)
    }
}
