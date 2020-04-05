use crate::sys::bigarray;
use crate::sys::caml_frame;
use crate::sys::mlvalues;

use std::marker::PhantomData;
use std::{mem, slice};

use crate::value::{Size, Value};

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

impl<'a, T> crate::FromValue for Array1<'a, T> {
    fn from_value(value: Value) -> Array1<'a, T> {
        Array1(value, PhantomData)
    }
}

impl<'a, T> crate::ToValue for Array1<'a, T> {
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
            x = unsafe {
                bigarray::caml_ba_alloc_dims(
                    T::kind() | bigarray::Managed::EXTERNAL as i32,
                    1,
                    data.as_mut_ptr() as bigarray::Data,
                    data.len() as i32,
                )
            };
            x
        });
        Array1(Value(x), PhantomData)
    }

    /// Create a new OCaml `Bigarray.Array1` with the given type and size
    pub fn create(n: Size) -> Array1<'a, T> {
        let x = caml_frame!(|x| {
            let data = unsafe { bigarray::malloc(n * mem::size_of::<T>()) };
            x = unsafe {
                bigarray::caml_ba_alloc_dims(
                    T::kind() | bigarray::Managed::MANAGED as i32,
                    1,
                    data,
                    n as mlvalues::Intnat,
                )
            };
            x
        });
        Array1(Value(x), PhantomData)
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
