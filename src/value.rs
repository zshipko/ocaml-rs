use core;
use tag::Tag;

pub type Size = core::mlvalues::Size;

#[derive(Debug, Clone)]
pub struct Value(pub core::mlvalues::Value);

impl From<Value> for core::mlvalues::Value {
    fn from(v: Value) -> core::mlvalues::Value {
        v.0
    }
}

impl Value {
    pub fn alloc(n: usize, tag: Tag) -> Value {
        let x = unsafe  {
            core::alloc::caml_alloc(n, tag as u8)
        };

        Value::new(x)
    }

    pub fn new(v: core::mlvalues::Value) -> Value {
        Value(v)
    }

    pub fn value(&self) -> core::mlvalues::Value {
        self.0
    }

    pub fn ptr<T>(p: *const T) -> Value {
        Value(p as core::mlvalues::Value)
    }

    pub fn int(i: i32) -> Value {
        Value(val_int!(i))
    }

    pub fn long(i: i64) -> Value {
        Value(val_long!(i))
    }

    pub fn double(d: f64) -> Value {
        unsafe {
            Value(core::alloc::caml_copy_double(d))
        }
    }

    pub fn is_block(&self) -> bool {
        core::mlvalues::is_block(self.0)
    }

    pub fn is_long(&self) -> bool {
        core::mlvalues::is_long(self.0)
    }

    pub fn field(&self, i: Size) -> Value {
        unsafe {
            Value::new(*core::mlvalues::field(self.0, i))
        }
    }

    pub fn store_field(&mut self, i: Size, val: Value) {
        unsafe {
            core::memory::store_field(self.0, i, val.0)
        }
    }

    pub fn int_val(&self) -> i32 {
        int_val!(self.0) as i32
    }

    pub fn long_val(&self) -> i64 {
        long_val!(self.0) as i64
    }

    pub fn double_val(&self) -> f64 {
        unsafe {
            *self.ptr_val::<f64>()
        }
    }

    pub fn ptr_val<T>(&self) -> *const T {
        self.0 as *mut T
    }

    pub fn call(&self, arg: Value) -> Value {
        unsafe {
            Value::new(core::callback::caml_callback(self.0, arg.0))
        }
    }
}
