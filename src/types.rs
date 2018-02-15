use mlvalues;
use memory;
use mlvalues::Value;
use alloc;
use tag::Tag;
use error::Error;

pub struct Tuple(Value, usize);

impl From<Tuple> for Value {
    fn from(t: Tuple) -> Value {
        t.0
    }
}

impl Tuple {
    pub unsafe fn new(n: usize) -> Tuple {
        let val = alloc::caml_alloc_tuple(n);
        Tuple(val, n)
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn value(&self) -> Value {
        self.0
    }

    pub unsafe fn set(&mut self, i: usize, v: Value) -> Result<(), Error> {
        if i < self.1 {
            memory::store_field(self.value(), i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub unsafe fn get(&self, i: usize) -> Result<Value, Error> {
        if i < self.1 {
            Ok(*mlvalues::field(self.value(), i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

pub struct Array(Value, usize);

impl From<Array> for Value {
    fn from(t: Array) -> Value {
        t.0
    }
}

impl Array {
    pub unsafe fn new(n: usize) -> Array {
        let val = alloc::caml_alloc(n, Tag::Zero as u8);
        Array(val, n)
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn value(&self) -> Value {
        self.0
    }

    pub unsafe fn set(&mut self, i: usize, v: Value) -> Result<(), Error> {
        if i < self.1 {
            memory::store_field(self.value(), i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub unsafe fn get(&self, i: usize) -> Result<Value, Error> {
        if i < self.1 {
            Ok(*mlvalues::field(self.value(), i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

