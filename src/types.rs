use crate::core::alloc;
use crate::core::bigarray;
use crate::core::mlvalues;
use crate::core::mlvalues::empty_list;
use crate::error::Error;
use crate::tag;

use std::marker::PhantomData;
use std::{mem, ptr, slice};

use crate::value::{Size, Value};

/// OCaml Tuple type
pub struct Tuple(Value, Size);

impl From<Tuple> for Value {
    fn from(t: Tuple) -> Value {
        t.0
    }
}

impl From<Value> for Tuple {
    fn from(v: Value) -> Tuple {
        let len = v.array_length();
        Tuple(v, len)
    }
}

impl<'a, V: crate::ToValue> From<&'a [V]> for Tuple {
    fn from(a: &'a [V]) -> Tuple {
        Tuple(
            caml_frame!(|t| {
                t.0 = unsafe { alloc::caml_alloc_tuple(a.len()) };
                for (n, i) in a.iter().enumerate() {
                    t.store_field(n, i.to_value());
                }
                t
            }),
            a.len(),
        )
    }
}

impl Tuple {
    /// Create a new tuple
    pub fn new(n: Size) -> Tuple {
        let x = caml_frame!(|x| {
            x.0 = unsafe { alloc::caml_alloc_tuple(n) };
            x
        });

        Tuple(x, n)
    }

    /// Tuple length
    pub fn len(&self) -> Size {
        self.1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
pub struct Array(Value);

impl From<Array> for Value {
    fn from(t: Array) -> Value {
        t.0
    }
}

impl From<Value> for Array {
    fn from(v: Value) -> Array {
        Array(v)
    }
}

impl<'a, V: crate::ToValue> From<&'a [V]> for Array {
    fn from(a: &'a [V]) -> Array {
        Array(caml_frame!(|x| {
            x.0 = unsafe { alloc::caml_alloc(a.len(), 0) };
            for (n, i) in a.iter().enumerate() {
                x.store_field(n, i.to_value());
            }
            x
        }))
    }
}

impl crate::ToValue for Array {
    fn to_value(&self) -> Value {
        self.0.to_value()
    }
}

impl Array {
    /// Create a new array of the given size
    pub fn new(n: Size) -> Array {
        let x = caml_frame!(|x| {
            x.0 = unsafe { alloc::caml_alloc(n, 0) };
            x
        });
        Array(x)
    }

    /// Check if Array contains only doubles
    pub fn is_double_array(&self) -> bool {
        unsafe { alloc::caml_is_double_array(self.0.value()) == 1 }
    }

    /// Array length
    pub fn len(&self) -> Size {
        unsafe { mlvalues::caml_array_length(self.0.value()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set value to double array
    pub fn set_double(&mut self, i: Size, f: f64) -> Result<(), Error> {
        if i < self.len() {
            if !self.is_double_array() {
                return Err(Error::NotDoubleArray);
            }

            unsafe {
                let ptr = self.0.ptr_val::<f64>().add(i) as *mut f64;
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

            Ok(unsafe { self.get_double_unchecked(i) })
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get a value from a double array without checking if the array is actually a double array
    unsafe fn get_double_unchecked(&mut self, i: Size) -> f64 {
        *self.0.ptr_val::<f64>().add(i)
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
            self.get_unchecked(i)
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get array index without bounds checking
    pub fn get_unchecked(&self, i: Size) -> Result<Value, Error> {
        Ok(self.0.field(i))
    }
}

/// OCaml list type
pub struct List(Value);

impl From<List> for Value {
    fn from(t: List) -> Value {
        t.0
    }
}

impl From<Value> for List {
    fn from(v: Value) -> List {
        List(v)
    }
}

impl<'a, V: crate::ToValue> From<&'a [V]> for List {
    fn from(a: &'a [V]) -> List {
        crate::list!(_ a.iter().map(|x| x.to_value()))
    }
}

impl crate::ToValue for List {
    fn to_value(&self) -> Value {
        self.0.to_value()
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

impl List {
    /// Create a new OCaml list
    pub fn new() -> List {
        List(caml_frame!(|x| { x }))
    }

    /// Returns the number of items in `self`
    pub fn len(&self) -> Size {
        let mut length = 0;
        let mut tmp = self.0.clone();
        while tmp.0 != empty_list() {
            tmp = tmp.field(1);
            length += 1;
        }
        length
    }

    /// Returns true when the list is empty
    pub fn is_empty(&self) -> bool {
        let item: usize = (self.0).0;
        item == empty_list()
    }

    /// Add an element to the front of the list
    pub fn push_hd(&mut self, v: Value) {
        let tmp = caml_frame!(|x, tmp| {
            x.0 = (self.0).0;

            tmp = Value::alloc_small(2, 0);
            tmp.store_field(0, v);
            tmp.store_field(1, x);
            tmp
        });

        self.0 = tmp;
    }

    /// List head
    pub fn hd(&self) -> Option<Value> {
        if self.is_empty() {
            return None;
        }

        Some(self.0.field(0))
    }

    /// List tail
    pub fn tl(&self) -> Value {
        self.0.field(1)
    }

    /// List as vector
    pub fn to_vec(&self) -> Vec<Value> {
        let mut vec: Vec<Value> = Vec::new();
        let mut tmp = self.0.clone();
        while tmp.0 != empty_list() {
            let val = tmp.field(0);
            vec.push(val);
            tmp = tmp.field(1);
        }
        vec
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
            let x = caml_frame!(|x| {
                x.0 = alloc::caml_alloc_string(len);
                let ptr = string_val!(x.0) as *mut u8;
                ptr::copy(s.as_ptr(), ptr, len);
                x
            });

            Str(x)
        }
    }
}

impl<'a> From<&'a [u8]> for Str {
    fn from(s: &'a [u8]) -> Str {
        unsafe {
            let len = s.len();
            let x = caml_frame!(|x| {
                x.0 = alloc::caml_alloc_string(len);
                let ptr = string_val!(x.0) as *mut u8;
                ptr::copy(s.as_ptr(), ptr, len);
                x
            });
            Str(x)
        }
    }
}

impl From<Value> for Str {
    fn from(v: Value) -> Str {
        if v.tag() != tag::STRING {
            panic!("Invalid string value, got tag {:?}", v.tag());
        } else {
            Str(v)
        }
    }
}

impl crate::ToValue for Str {
    fn to_value(&self) -> Value {
        self.0.to_value()
    }
}

impl Str {
    /// Create a new string of a given length
    pub fn new(n: Size) -> Str {
        Str(caml_frame!(|s| {
            s.0 = unsafe { alloc::caml_alloc_string(n) };
            s
        }))
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

impl<T: BigarrayKind> crate::ToValue for Array1<T> {
    fn to_value(&self) -> Value {
        self.0.to_value()
    }
}

impl<T: Copy + BigarrayKind> From<&[T]> for Array1<T> {
    fn from(x: &[T]) -> Array1<T> {
        let mut arr = Array1::<T>::create(x.len());
        let data = arr.data_mut();
        for (n, i) in x.iter().enumerate() {
            data[n] = *i;
        }
        arr
    }
}

impl<T: BigarrayKind> Array1<T> {
    /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
    /// the `data` parameter must outlive the resulting bigarray or there is
    /// no guarantee the data will be valid. Use `Array1::from` to clone the
    /// contents of a slice.
    pub fn of_slice(data: &mut [T]) -> Array1<T> {
        let x = caml_frame!(|x| {
            x.0 = unsafe {
                bigarray::caml_ba_alloc_dims(
                    T::kind() | bigarray::Managed::EXTERNAL as i32,
                    1,
                    data.as_mut_ptr() as bigarray::Data,
                    data.len() as i32,
                )
            };
            x
        });
        Array1(x, PhantomData)
    }

    /// Create a new OCaml `Bigarray.Array1` with the given type and size
    pub fn create(n: Size) -> Array1<T> {
        let x = caml_frame!(|x| {
            let data = unsafe { bigarray::malloc(n * mem::size_of::<T>()) };
            x.0 = unsafe {
                bigarray::caml_ba_alloc_dims(
                    T::kind() | bigarray::Managed::MANAGED as i32,
                    1,
                    data,
                    n as mlvalues::Intnat,
                )
            };
            x
        });
        Array1(x, PhantomData)
    }

    /// Returns the number of items in `self`
    pub fn len(&self) -> Size {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { (*ba).dim as usize }
    }

    /// Returns true when `self.len() == 0`
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get underlying data as Rust slice
    pub fn data(&self) -> &[T] {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { slice::from_raw_parts((*ba).data as *const T, self.len()) }
    }

    /// Get underlying data as mutable Rust slice
    pub fn data_mut(&mut self) -> &mut [T] {
        let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
        unsafe { slice::from_raw_parts_mut((*ba).data as *mut T, self.len()) }
    }
}
