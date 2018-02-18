use core::mlvalues;
use core::mlvalues::empty_list;
use core::memory;
use core::alloc;
use tag::Tag;
use error::Error;

use std::ptr;

use value::{Size, Value};

pub struct Tuple(Value, Size);

impl From<Tuple> for Value {
    fn from(t: Tuple) -> Value {
        t.0
    }
}

impl <R: AsRef<[Value]>> From<R> for Tuple {
    fn from(t: R) -> Tuple {
        let mut dst = Tuple::new(t.as_ref().len());

        for (n, item) in t.as_ref().iter().enumerate() {
            let _ = dst.set(n, item.clone());
        }

        dst
    }
}

impl Tuple {
    pub fn new(n: Size) -> Tuple {
        unsafe {
            let val = Value::new(alloc::caml_alloc_tuple(n));
            Tuple(val, n)
        }
    }

    pub fn len(&self) -> Size {
        self.1
    }

    pub fn set(&mut self, i: Size, v: Value) -> Result<(), Error> {
        if i < self.1 {
            self.0.store_field(i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub fn get(&self, i: Size) -> Result<Value, Error> {
        if i < self.1 {
            Ok(self.0.field(i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

pub struct Array(Value, Size);

impl From<Array> for Value {
    fn from(t: Array) -> Value {
        t.0
    }
}

impl <R: AsRef<[Value]>> From<R> for Array {
    fn from(t: R) -> Array {
        let mut dst = Array::new(t.as_ref().len());

        for (n, item) in t.as_ref().iter().enumerate() {
            let _ = dst.set(n, item.clone());
        }

        dst
    }
}

impl From<Value> for Array {
    fn from(v: Value) -> Array {
        if !v.is_block() {
            let mut arr = Array::new(1);
            let _ = arr.set(0, v);
            arr
        } else {
            let length = unsafe { mlvalues::caml_array_length(v.0) };
            Array(v, length)
        }
    }
}

impl Array {
    pub fn new(n: Size) -> Array {
        unsafe {
            let val = alloc::caml_alloc(n, Tag::Zero as u8);
            Array(Value::new(val), n)
        }
    }

    pub fn len(&self) -> Size {
        self.1
    }

    pub fn set(&mut self, i: Size, v: Value) -> Result<(), Error> {
        if i < self.1 {
            self.0.store_field(i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub fn get(&self, i: Size) -> Result<Value, Error> {
        if i < self.1 {
            Ok(self.0.field(i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

pub struct List(Value, Size);

impl From<List> for Value {
    fn from(t: List) -> Value {
        t.0
    }
}

impl <R: AsRef<[Value]>> From<R> for List {
    fn from(t: R) -> List {
        let mut dst = List::new();

        for item in t.as_ref().iter().rev() {
            let _ = dst.push_hd(item.clone());
        }

        dst
    }
}

impl List {
    pub fn new() -> List {
        List(Value::new(empty_list()), 0)
    }

    pub fn len(&self) -> Size {
        self.1
    }

    pub fn push_hd(&mut self, v: Value) {
        unsafe {
            let tmp = alloc::caml_alloc(2, 0);
            memory::store_field(tmp, 0, v.0);
            memory::store_field(tmp, 1, (self.0).0);
            self.0 = Value::new(tmp);
            self.1 += 1;
        }
    }

    pub fn hd(&self) -> Option<Value> {
        if self.1 == 0 {
            return None
        }

        Some(self.0.field(0))
    }

    pub fn tl(&self) -> Value {
        if self.1 == 0 {
            Value::new(empty_list())
        } else {
            self.0.field(1)
        }
    }
}

pub struct Str(Value, Size);

impl From<Str> for Value {
    fn from(t: Str) -> Value {
        t.0
    }
}

impl <'a> From<&'a str> for Str {
    fn from(s: &'a str) -> Str {
        unsafe {
            let len = s.len();
            let x = alloc::caml_alloc_string(len);
            let ptr = string_val!(x) as *mut u8;
            ptr::copy(s.as_ptr(), ptr, len);
            Str(Value::new(x), len)
        }
    }
}

impl From<Value> for Str {
    fn from(v: Value) -> Str {
        unsafe {
            let len = mlvalues::caml_string_length(v.0);
            Str(v, len)
        }
    }
}

impl Str {
    pub fn new(n: Size) -> Str {
        unsafe {
            let s = alloc::caml_alloc_string(n);
            Str(Value::new(s), n)
        }
    }

    pub fn len(&self) -> Size {
        self.1
    }

    pub fn as_str(&self) -> &str {
        let ptr = string_val!((self.0).0);
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, self.1);
            ::std::str::from_utf8_unchecked(slice)
        }
    }
}
