use crate::caml_frame;
use crate::core::bigarray;
use crate::core::mlvalues;
use crate::core::mlvalues::EMPTY_LIST;

use std::marker::PhantomData;
use std::{mem, slice};

use crate::value::{Size, Value};

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

/*impl<'a, V: crate::ToValue> From<&'a [V]> for List {
    fn from(a: &'a [V]) -> List {
        crate::ToValue::to_value(a)
    }
}*/

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
        let mut tmp = self.0;
        while tmp.0 != EMPTY_LIST {
            tmp = tmp.field(1);
            length += 1;
        }
        length
    }

    /// Returns true when the list is empty
    pub fn is_empty(&self) -> bool {
        let item: usize = (self.0).0;
        item == EMPTY_LIST
    }

    /// Add an element to the front of the list
    pub fn push_hd(&mut self, v: Value) {
        let tmp = caml_frame!(|x, tmp| {
            x.0 = (self.0).0;

            let mut y = crate::alloc_small(2, 0);
            y.store_field(0, v);
            y.store_field(1, x);
            tmp.0 = y.0;
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
        let mut tmp = self.0;
        while tmp.0 != EMPTY_LIST {
            let val = tmp.field(0);
            vec.push(val);
            tmp = tmp.field(1);
        }
        vec
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
pub struct Array1<'a, T>(Value, PhantomData<&'a T>);

impl<'a, T: BigarrayKind> From<Array1<'a, T>> for Value {
    fn from(t: Array1<T>) -> Value {
        t.0
    }
}

impl<'a, T: BigarrayKind> From<Value> for Array1<'a, T> {
    fn from(v: Value) -> Array1<'a, T> {
        Array1(v, PhantomData)
    }
}

impl<'a, T: BigarrayKind> crate::ToValue for Array1<'a, T> {
    fn to_value(&self) -> Value {
        self.0.to_value()
    }
}

impl<'a, T: 'a + Copy + BigarrayKind> From<&'a [T]> for Array1<'a, T> {
    fn from(x: &[T]) -> Array1<'a, T> {
        let mut arr = Array1::<T>::create(x.len());
        let data = arr.data_mut();
        for (n, i) in x.iter().enumerate() {
            data[n] = *i;
        }
        arr
    }
}

impl<'a, T: BigarrayKind> Array1<'a, T> {
    /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
    /// the `data` parameter must outlive the resulting bigarray or there is
    /// no guarantee the data will be valid. Use `Array1::from` to clone the
    /// contents of a slice.
    pub fn of_slice(data: &'a mut [T]) -> Array1<'a, T> {
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
    pub fn create(n: Size) -> Array1<'a, T> {
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
