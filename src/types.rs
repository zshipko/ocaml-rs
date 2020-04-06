use crate::sys::{alloc, bigarray, caml_frame, mlvalues};
use crate::Error;

use std::marker::PhantomData;
use std::{mem, slice};

use crate::value::{FromValue, Size, ToValue, Value};

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Pointer<'a, T>(*mut T, PhantomData<&'a T>);

impl<'a, T: ToValue + FromValue> ToValue for Pointer<'a, T> {
    fn to_value(&self) -> Value {
        Value::ptr(self.0)
    }
}

impl<'a, T: ToValue + FromValue> FromValue for Pointer<'a, T> {
    fn from_value(value: Value) -> Self {
        let ptr = value.custom_ptr_val_mut();
        Pointer(ptr, PhantomData)
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T: ToValue + FromValue>(Value, PhantomData<&'a T>);

impl<'a, T: ToValue + FromValue> ToValue for Array<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

impl<'a, T: ToValue + FromValue> FromValue for Array<'a, T> {
    fn from_value(value: Value) -> Self {
        Array(value, PhantomData)
    }
}

impl<'a, T: ToValue + FromValue> Array<'a, T> {
    pub fn alloc(n: usize) -> Array<'a, T> {
        let x = caml_frame!(|x| {
            x = unsafe { alloc::caml_alloc(n, 0) };
            x
        });
        Array(Value(x), PhantomData)
    }

    /// Check if Array contains only doubles
    pub fn is_double_array(&self) -> bool {
        unsafe { alloc::caml_is_double_array((self.0).0) == 1 }
    }

    /// Array length
    pub fn len(&self) -> usize {
        unsafe { mlvalues::caml_array_length((self.0).0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set value to double array
    pub fn set_double(&mut self, i: usize, f: f64) -> Result<(), Error> {
        if i >= self.len() {
            return Err(Error::OutOfBounds);
        }

        if !self.is_double_array() {
            return Err(Error::NotDoubleArray);
        }

        unsafe {
            let ptr = self.0.ptr_val::<f64>().add(i) as *mut f64;
            *ptr = f;
        };

        Ok(())
    }

    /// Get a value from a double array
    pub fn get_double(&self, i: usize) -> Result<f64, Error> {
        if i >= self.len() {
            return Err(Error::OutOfBounds);
        }
        if !self.is_double_array() {
            return Err(Error::NotDoubleArray);
        }

        Ok(unsafe { self.get_double_unchecked(i) })
    }

    /// Get a value from a double array without checking if the array is actually a double array
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_double_unchecked(&self, i: usize) -> f64 {
        *self.0.ptr_val::<f64>().add(i)
    }

    /// Set array index
    pub fn set(&mut self, i: usize, v: T) -> Result<(), Error> {
        if i >= self.len() {
            return Err(Error::OutOfBounds);
        }
        self.0.store_field(i, v);
        Ok(())
    }

    /// Get array index
    pub fn get(&self, i: usize) -> Result<T, Error> {
        if i >= self.len() {
            return Err(Error::OutOfBounds);
        }
        Ok(unsafe { self.get_unchecked(i) })
    }

    /// Get array index without bounds checking
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, i: usize) -> T {
        T::from_value(self.0.field(i))
    }

    /// List as `Vec`
    pub fn to_vec(&self) -> Vec<T> {
        FromValue::from_value(self.to_value())
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct List<'a, T: ToValue + FromValue>(Value, PhantomData<&'a T>);

impl<'a, T: ToValue + FromValue> ToValue for List<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

impl<'a, T: ToValue + FromValue> FromValue for List<'a, T> {
    fn from_value(value: Value) -> Self {
        List(value, PhantomData)
    }
}

impl<'a, T: ToValue + FromValue> List<'a, T> {
    #[inline(always)]
    pub fn empty() -> List<'a, T> {
        List(Value::UNIT, PhantomData)
    }

    /// Returns the number of items in `self`
    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut tmp = self.0;
        while tmp.0 != mlvalues::EMPTY_LIST {
            tmp = tmp.field(1);
            length += 1;
        }
        length
    }

    /// Add an element to the front of the list
    pub fn push_hd(&mut self, v: T) {
        let tmp = caml_frame!(|x, tmp| {
            x = (self.0).0;
            unsafe {
                tmp = crate::sys::alloc::caml_alloc_small(2, 0);
                crate::sys::memory::store_field(tmp, 0, v.to_value().0);
                crate::sys::memory::store_field(tmp, 1, x);
                tmp
            }
        });

        (self.0).0 = tmp;
    }

    /// List head
    pub fn hd(&self) -> Option<T> {
        if self.0 == Self::empty().0 {
            return None;
        }

        Some(self.0.field(0))
    }

    /// List tail
    pub fn tl(&self) -> List<T> {
        if self.0 == Self::empty().0 {
            return Self::empty();
        }

        self.0.field(1)
    }

    /// List as `Vec`
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        let mut value = self.0;
        let empty = Self::empty().0;
        while value != empty {
            let val = value.field(0);
            vec.push(T::from_value(val));
            value = value.field(1);
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

impl<'a, T> crate::FromValue for Array1<'a, T> {
    fn from_value(value: Value) -> Array1<'a, T> {
        Array1(value, PhantomData)
    }
}

impl<'a, T> crate::ToValue for Array1<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

impl<'a, T: 'a + Copy + BigarrayKind> From<&'a mut [T]> for Array1<'a, T> {
    fn from(x: &'a mut [T]) -> Array1<'a, T> {
        Array1::of_slice(x)
    }
}

impl<'a, T: 'a + Copy + BigarrayKind> From<Vec<T>> for Array1<'a, T> {
    fn from(x: Vec<T>) -> Array1<'a, T> {
        let mut arr = Array1::<T>::create(x.len());
        let data = arr.data_mut();
        data.copy_from_slice(x.as_slice());
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
