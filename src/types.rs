use crate::core::alloc;
use crate::core::bigarray;
//use crate::core::memory;
use crate::core::mlvalues;
use crate::core::mlvalues::empty_list;
use crate::error::Error;
use crate::tag::Tag;

use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::slice;

use crate::value::{Size, Value};

/// OCaml Tuple type
pub struct Tuple<'a>(&'a mut Value, Size);

impl<'a> From<Tuple<'a>> for Value {
    fn from(t: Tuple<'a>) -> Value {
        t.0.clone()
    }
}

impl<'a> Tuple<'a> {
    /// Create a new tuple
    pub fn new(local: &'a mut Value, n: Size) -> Tuple<'a> {
        local.0 = unsafe { alloc::caml_alloc_tuple(n) };
        Tuple(local, n)
    }

    /// Tuple length
    pub fn len(&self) -> Size {
        self.1
    }

    /// Set tuple index
    pub fn set(&mut self, i: Size, v: Value) -> Result<(), Error> {
        if i < self.1 {
            self.0.store_field(i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get tuple index
    pub fn get(&self, i: Size) -> Result<Value, Error> {
        if i < self.1 {
            Ok(self.0.field(i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

/// OCaml Array type
pub struct Array<'a>(&'a mut Value);

impl<'a> From<Array<'a>> for Value {
    fn from(t: Array<'a>) -> Value {
        t.0.clone()
    }
}

impl<'a> From<&'a mut Value> for Array<'a> {
    fn from(v: &'a mut Value) -> Array<'a> {
        Array(v)
    }
}

impl<'a> Array<'a> {
    /// Create a new array of the given size
    pub fn new(value: &'a mut Value, n: Size) -> Array<'a> {
        unsafe {
            value.0 = alloc::caml_alloc(n, Tag::Zero.into());
            Array(value)
        }
    }

    /// Check if Array contains only doubles
    pub fn is_double_array(&self) -> bool {
        unsafe { alloc::caml_is_double_array(self.0.value()) == 1 }
    }

    /// Array length
    pub fn len(&self) -> Size {
        unsafe { mlvalues::caml_array_length(self.0.value()) }
    }

    /// Set value to double array
    pub fn set_double(&mut self, i: Size, f: f64) -> Result<(), Error> {
        if i < self.len() {
            if !self.is_double_array() {
                return Err(Error::NotDoubleArray);
            }

            unsafe {
                let ptr = self.0.ptr_val::<f64>().offset(i as isize) as *mut f64;
                *ptr = f;
            };

            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get a value from a double array
    pub fn get_double(&mut self, i: Size) -> Result<f64, Error> {
        if i < self.len() {
            if !self.is_double_array() {
                return Err(Error::NotDoubleArray);
            }

            unsafe { Ok(*self.0.ptr_val::<f64>().offset(i as isize)) }
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Set array index
    pub fn set(&mut self, i: Size, v: Value) -> Result<(), Error> {
        if i < self.len() {
            self.0.store_field(i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get array index
    pub fn get(&self, i: Size) -> Result<Value, Error> {
        if i < self.len() {
            Ok(self.0.field(i))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

/// OCaml list type
pub struct List<'a>(&'a mut Value);

impl<'a> From<List<'a>> for Value {
    fn from(t: List<'a>) -> Value {
        t.0.clone()
    }
}

impl<'a> From<&'a mut Value> for List<'a> {
    fn from(v: &'a mut Value) -> List {
        List(v)
    }
}

impl<'a> List<'a> {
    /// Create a new OCaml list
    pub fn new(local: &mut Value) -> List {
        List(local)
    }

    /// List length
    pub fn len(&self) -> Size {
        let mut length = 0;
        let mut tmp = self.0.clone();

        while tmp.0 != empty_list() {
            tmp = tmp.field(1);
            length += 1;
        }

        length
    }

    /// Add an element to the front of the list
    pub fn push_hd(&mut self, local: &mut Value, v: Value) {
        local.0 = Value::alloc(2, Tag::Zero).0;
        local.store_field(0, v);
        local.store_field(1, self.0.clone());
        (self.0).0 = local.0
    }

    /// List head
    pub fn hd(&self) -> Option<Value> {
        if self.len() == 0 {
            return None;
        }

        Some(self.0.field(0))
    }

    /// List tail
    pub fn tl(&self) -> Value {
        self.0.field(1)
    }
}

/// OCaml String type
pub struct Str(Value);

impl From<Str> for Value {
    fn from(t: Str) -> Value {
        t.0
    }
}

impl<'a> From<&'a str> for Str {
    fn from(s: &'a str) -> Str {
        unsafe {
            let len = s.len();
            let x = alloc::caml_alloc_string(len);
            let ptr = string_val!(x) as *mut u8;
            ptr::copy(s.as_ptr(), ptr, len);
            Str(Value::new(x))
        }
    }
}

impl<'a> From<&'a [u8]> for Str {
    fn from(s: &'a [u8]) -> Str {
        unsafe {
            let len = s.len();
            let x = alloc::caml_alloc_string(len);
            let ptr = string_val!(x) as *mut u8;
            ptr::copy(s.as_ptr(), ptr, len);
            Str(Value::new(x))
        }
    }
}

impl From<Value> for Str {
    fn from(v: Value) -> Str {
        if v.tag() != Tag::String {
            panic!("Invalid string value, got tag {:?}", v.tag());
        } else {
            Str(v)
        }
    }
}

impl Str {
    /// Create a new string of a given length
    pub fn new(n: Size) -> Str {
        unsafe {
            let s = alloc::caml_alloc_string(n);
            Str(Value::new(s))
        }
    }

    /// String length
    pub fn len(&self) -> Size {
        unsafe { mlvalues::caml_string_length(self.0.value()) }
    }

    /// Check if a string is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Access OCaml string as `&str`
    pub fn as_str(&self) -> &str {
        let ptr = string_val!((self.0).0);
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, self.len());
            ::std::str::from_utf8_unchecked(slice)
        }
    }

    /// Access OCaml string as `&mut str`
    pub fn as_str_mut(&mut self) -> &mut str {
        let ptr = string_val!((self.0).0) as *mut u8;
        unsafe {
            let slice = ::std::slice::from_raw_parts_mut(ptr, self.len());
            ::std::str::from_utf8_unchecked_mut(slice)
        }
    }

    /// Access OCaml string as `&[u8]`
    pub fn data(&self) -> &[u8] {
        let ptr = string_val!((self.0).0);
        unsafe { ::std::slice::from_raw_parts(ptr, self.len()) }
    }

    /// Access OCaml string as `&mut [u8]`
    pub fn data_mut(&mut self) -> &mut [u8] {
        let ptr = string_val!((self.0).0) as *mut u8;
        unsafe { ::std::slice::from_raw_parts_mut(ptr, self.len()) }
    }
}

pub trait BigarrayKind {
    type T;
    fn kind() -> i32;
}

macro_rules! make_kind {
    ($t:ty, $k:ident) => {
        impl BigarrayKind for $t {
            type T = $t;

            fn kind() -> i32 {
                bigarray::Kind::$k as i32
            }
        }
    };
}

make_kind!(u8, UINT8);
make_kind!(i8, SINT8);
make_kind!(u16, UINT16);
make_kind!(i16, SINT16);
make_kind!(f32, FLOAT32);
make_kind!(f64, FLOAT64);
make_kind!(i64, INT64);
make_kind!(i32, INT32);
make_kind!(char, CHAR);

/// OCaml Bigarray.Array1 type
pub struct Array1<T>(Value, PhantomData<T>);

impl<T: BigarrayKind> From<Array1<T>> for Value {
    fn from(t: Array1<T>) -> Value {
        t.0
    }
}

impl<T: BigarrayKind> From<Value> for Array1<T> {
    fn from(v: Value) -> Array1<T> {
        Array1(v, PhantomData)
    }
}

impl<T: BigarrayKind> Array1<T> {
    pub fn of_slice(data: &mut [T]) -> Array1<T> {
        unsafe {
            let s = bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::EXTERNAL as i32,
                1,
                data.as_mut_ptr() as bigarray::Data,
                data.len() as i32,
            );
            Array1(Value::new(s), PhantomData)
        }
    }

    pub fn create(n: Size) -> Array1<T> {
        unsafe {
            let data = bigarray::malloc(n * mem::size_of::<T>());
            let s = bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::MANAGED as i32,
                1,
                data,
                n as mlvalues::Intnat,
            );
            Array1(Value::new(s), PhantomData)
        }
    }

    pub fn len(&self) -> Size {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { (*ba).dim as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> &[T] {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { slice::from_raw_parts((*ba).data as *const T, self.len()) }
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { slice::from_raw_parts_mut((*ba).data as *mut T, self.len()) }
    }
}
