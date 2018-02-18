use core;
use tag::Tag;
use error::Error;

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

impl Value {
    /// Allocate a new value with the given size and tag
    pub fn alloc(n: usize, tag: Tag) -> Value {
        let x = unsafe  {
            core::alloc::caml_alloc(n, tag as u8)
        };

        Value::new(x)
    }

    /// Create a new Value from an existing OCaml `value`
    pub fn new(v: core::mlvalues::Value) -> Value {
        Value(v)
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

    /// Create a new opaque pointer Value
    pub fn ptr<T>(p: *const T) -> Value {
        Value(p as core::mlvalues::Value)
    }

    /// Create an integer Value
    pub fn int(i: i32) -> Value {
        Value(val_int!(i))
    }

    /// Create a long Value
    pub fn long(i: i64) -> Value {
        Value(val_long!(i))
    }

    /// Create a double Value
    pub fn double(d: f64) -> Value {
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
    pub fn store_field(&mut self, i: Size, val: Value) {
        unsafe {
            core::memory::store_field(self.0, i, val.0)
        }
    }

    /// Convert an OCaml integer to `i32`
    pub fn int_val(&self) -> i32 {
        int_val!(self.0) as i32
    }

    /// Convert an OCaml integer to `i64`
    pub fn long_val(&self) -> i64 {
        long_val!(self.0) as i64
    }

    /// Convert an OCaml float to `f64`
    pub fn double_val(&self) -> f64 {
        unsafe {
            *self.ptr_val::<f64>()
        }
    }

    /// Get a pointer stored in an opaque value
    pub fn ptr_val<T>(&self) -> *const T {
        self.0 as *mut T
    }

    /// Call a closure with a single argument
    pub fn call(&self, arg: Value) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback(self.0, arg.0)))
        }
    }

    /// Call a closure with two arguments
    pub fn call2(&self, arg1: Value, arg2: Value) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback2(self.0, arg1.0, arg2.0)))
        }
    }

    /// Call a closure with three arguments
    pub fn call3(&self, arg1: Value, arg2: Value, arg3: Value) -> Result<Value, Error> {
        if self.tag() != Tag::Closure  {
            return Err(Error::NotCallable)
        }

        unsafe {
            Ok(Value::new(core::callback::caml_callback3(self.0, arg1.0, arg2.0, arg3.0)))
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
}
