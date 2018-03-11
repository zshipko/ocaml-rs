use core;
use tag::Tag;
use error::Error;
use runtime::hash_variant;

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = core::mlvalues::Size;

/// Value wraps the native OCaml `value` type
#[derive(Debug, Clone)]
pub struct Value(pub core::mlvalues::Value);

impl From<Value> for core::mlvalues::Value {
    fn from(v: Value) -> core::mlvalues::Value {
        v.0
    }
}

pub trait ToValue {
    fn to_value(&self) -> Value;
}

pub trait FromValue {
    fn from_value(v: Value) -> Self;
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl FromValue for Value {
    fn from_value(v: Value) -> Value {
        v
    }
}

pub const TRUE: Value = Value(val_int!(1));
pub const FALSE: Value = Value(val_int!(0));
pub const NONE: Value = Value(val_int!(0));
pub const UNIT: Value = Value(core::mlvalues::UNIT);

impl Value {
    /// Allocate a new value with the given size and tag
    pub fn alloc(n: usize, tag: Tag) -> Value {
        let x = unsafe  {
            core::alloc::caml_alloc(n, tag.into())
        };

        Value::new(x)
    }

    /// Allocate a new small value with the given size and tag
    pub fn alloc_small(n: usize, tag: Tag) -> Value {
        let x = unsafe  {
            core::alloc::caml_alloc_small(n, tag.into())
        };

        Value::new(x)
    }

    /// Allocate a new value with a custom finalizer
    pub fn alloc_final(n: usize, finalizer: extern "C" fn(core::Value)) -> Value {
        let x = unsafe {
            core::alloc::caml_alloc_final(n, finalizer, 0, 1)
        };

        Value::new(x)
    }

    /// Create a new Value from an existing OCaml `value`
    pub fn new(v: core::mlvalues::Value) -> Value {
        Value(v)
    }

    /// See caml_register_global_root
    pub fn register_global_root(&self) {
        unsafe {
            core::memory::caml_register_global_root(&self.value())
        }
    }

    /// Set caml_remove_global_root
    pub fn remove_global_root(&self) {
        unsafe {
            core::memory::caml_remove_global_root(&self.value())
        }
    }

    /// Get the underlying OCaml `value`
    pub fn value(&self) -> core::mlvalues::Value {
        self.0
    }

    /// Get the tag for the underlying OCaml `value`
    pub fn tag(&self) -> Tag {
        unsafe {
            Tag::new(tag_val!(self.0))
        }
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
        let mut x = Self::alloc(1, Tag::Zero);
        x.store_field(0, v.to_value());
        x
    }

    /// OCaml None value
    pub fn none() -> Value {
        NONE
    }

    /// Create a variant value
    pub fn variant<V: ToValue>(tag: u8, value: Option<V>) -> Value {
        match value {
            Some(v) => {
                let mut x = Self::alloc(1, Tag::Tag(tag));
                x.store_field(0, v.to_value());
                x
            },
            None => Self::alloc(0, Tag::Tag(tag))
        }
    }

    /// Create a new opaque pointer Value
    pub fn ptr<T>(p: *const T) -> Value {
        Value(p as core::mlvalues::Value)
    }

    /// Create an integer Value from `i32`
    pub fn i32(i: i32) -> Value {
        Value(val_int!(i))
    }

    /// Create an integer Value from `i64`
    pub fn i64(i: i64) -> Value {
        Value(val_int!(i))
    }

    /// Create an OCaml int64 from `i64`
    pub fn int64(i: i64) -> Value {
        unsafe {
            Value::new(core::alloc::caml_copy_int64(i))
        }
    }

    /// Create an OCaml int32 from `i32`
    pub fn int32(i: i32) -> Value {
        unsafe {
            Value::new(core::alloc::caml_copy_int32(i))
        }
    }

    /// Create an OCaml native int from `isize`
    pub fn nativeint(i: isize) -> Value {
        unsafe {
            Value::new(core::alloc::caml_copy_nativeint(i))
        }
    }

    /// Create a long Value from `isize`
    pub fn isize(i: isize) -> Value {
        Value(val_long!(i))
    }

    /// Create a value from `usize`
    pub fn usize(u: usize) -> Value {
        Value(val_long!(u))
    }

    /// Create a value from `f64`
    pub fn f64(d: f64) -> Value {
        unsafe {
            Value(core::alloc::caml_copy_double(d))
        }
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub fn is_block(&self) -> bool {
        core::mlvalues::is_block(self.0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub fn is_long(&self) -> bool {
        core::mlvalues::is_long(self.0)
    }

    /// Get index of underlying OCaml block value
    pub fn field(&self, i: Size) -> Value {
        unsafe {
            Value::new(*core::mlvalues::field(self.0, i))
        }
    }

    /// Set index of underlying OCaml block value
    pub fn store_field<V: ToValue>(&mut self, i: Size, val: V) {
        unsafe {
            core::memory::store_field(self.0, i, val.to_value().0)
        }
    }

    /// Convert an OCaml integer to `i32`
    pub fn i32_val(&self) -> i32 {
        int_val!(self.0) as i32
    }

    /// Convert an OCaml integer to `i64`
    pub fn i64_val(&self) -> i64 {
        int_val!(self.0) as i64
    }

    /// Convert an OCaml integer to `isize`
    pub fn isize_val(&self) -> isize {
        long_val!(self.0) as isize
    }

    /// Convert an OCaml integer to `usize`
    pub fn usize_val(&self) -> usize {
        long_val!(self.0) as usize
    }

    /// Convert an OCaml float to `f64`
    pub fn f64_val(&self) -> f64 {
        unsafe {
            *self.ptr_val::<f64>()
        }
    }

    /// Convert an OCaml int32 to `i32`
    pub fn int32_val(&self) -> i32 {
        unsafe {
            *self.custom_ptr_val::<i32>()
        }
    }

    /// Convert an OCaml int64 to `i64`
    pub fn int64_val(&self) -> i64 {
        unsafe {
            *self.custom_ptr_val::<i64>()
        }
    }

    /// Convert an OCaml integer to `isize`
    pub fn nativeint_val(&self) -> isize {
        unsafe {
            *self.custom_ptr_val::<isize>()
        }
    }

    pub fn custom_ptr_val<T>(&self) -> *const T {
        unsafe {
            core::mlvalues::field(self.0, 1) as *const T
        }
    }

    /// Get a pointer stored in an opaque value
    pub fn ptr_val<T>(&self) -> *const T {
        self.0 as *const T
    }

    /// Get a pointer stored in an opaque value
    pub fn mut_ptr_val<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    /// Call a closure with a single argument
    pub fn call<A: ToValue>(&self, arg: A) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback(self.0, arg.to_value().0)))
        }
    }

    /// Call a closure with two arguments
    pub fn call2<A: ToValue, B: ToValue>(&self, arg1: A, arg2: B) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback2(self.0, arg1.to_value().0, arg2.to_value().0)))
        }
    }

    /// Call a closure with three arguments
    pub fn call3<A: ToValue, B: ToValue, C: ToValue>(&self, arg1: A, arg2: B, arg3: C) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback3(self.0, arg1.to_value().0, arg2.to_value().0, arg3.to_value().0)))
        }
    }

    /// Call a closure with `n` arguments
    pub fn call_n<A: AsRef<[Value]>>(&self, args: A) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        let n = args.as_ref().len();
        let x: Vec<core::mlvalues::Value> = args.as_ref().iter().map(|x| x.0).collect();
        unsafe {
            Ok(Value::new(core::callback::caml_callbackN(self.0, n, x.as_ptr() as *mut core::mlvalues::Value)))
        }
    }

    /// Call a closure with a single argument, returning an exception value
    pub fn call_exn<A: ToValue>(&self, arg: A) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        let v = unsafe {
            core::callback::caml_callback_exn(self.0, arg.to_value().0)
        };

        if is_exception_result!(v) {
            Err(Error::Exception(Value::new(extract_exception!(v))))
        } else {
            Ok(Value::new(v))
        }
    }

    /// Call a closure with two arguments, returning an exception value
    pub fn call2_exn<A: ToValue, B: ToValue>(&self, arg1: A, arg2: B) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        let v = unsafe {
            core::callback::caml_callback2(self.0, arg1.to_value().0, arg2.to_value().0)
        };

        if is_exception_result!(v) {
            Err(Error::Exception(Value::new(extract_exception!(v))))
        } else {
            Ok(Value::new(v))
        }
    }

    /// Call a closure with three arguments, returning an exception value
    pub fn call3_exn<A: ToValue, B: ToValue, C: ToValue>(&self, arg1: A, arg2: B, arg3: C) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        let v = unsafe {
            core::callback::caml_callback3(self.0, arg1.to_value().0, arg2.to_value().0, arg3.to_value().0)
        };

        if is_exception_result!(v) {
            Err(Error::Exception(Value::new(extract_exception!(v))))
        } else {
            Ok(Value::new(v))
        }
    }

    /// Call a closure with `n` arguments, returning an exception value
    pub fn call_n_exn<A: AsRef<[Value]>>(&self, args: A) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        let n = args.as_ref().len();
        let x: Vec<core::mlvalues::Value> = args.as_ref().iter().map(|x| x.0).collect();
        let v = unsafe {
            core::callback::caml_callbackN(self.0, n, x.as_ptr() as *mut core::mlvalues::Value)
        };

        if is_exception_result!(v) {
            Err(Error::Exception(Value::new(extract_exception!(v))))
        } else {
            Ok(Value::new(v))
        }
    }

    /// Modify an OCaml value in place
    pub fn modify<V: ToValue>(&mut self, v: V) {
        unsafe {
            core::memory::caml_modify(&mut self.0, v.to_value().0)
        }
    }

    /// Determines if the current value is an exception
    pub fn is_exception_result(&self) -> bool {
        is_exception_result!(self.value())
    }

    /// Get object method
    pub fn method<S: AsRef<str>>(&self, name: S) -> Option<Value> {
        if self.tag() != Tag::Object {
            return None
        }

        let v = unsafe {
            core::mlvalues::caml_get_public_method(self.value(), hash_variant(name).value())
        };

        if v == 0 {
            return None
        }

        Some(Value::new(v))
    }
}


