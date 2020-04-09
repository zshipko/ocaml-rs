use crate::sys::{alloc, mlvalues};
use crate::Error;

use std::marker::PhantomData;
use std::{mem, slice};

use crate::value::{FromValue, Size, ToValue, Value};

/// An opaque handle to a Rust value
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Opaque<'a, T>(Value, PhantomData<&'a T>);

unsafe impl<'a, T> ToValue for Opaque<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

unsafe impl<'a, T> FromValue for Opaque<'a, T> {
    fn from_value(value: Value) -> Self {
        Opaque(value, PhantomData)
    }
}

extern "C" fn ignore(_: Value) {}

impl<'a, T> Opaque<'a, T> {
    /// Allocate a new value with an optional custom finalizer
    /// NOTE: `value` will be copied into memory allocated by the OCaml runtime, you are
    /// responsible for managing the lifetime of the value on the Rust side
    ///
    /// # Safety
    /// This function passes Rust data into OCaml, anything can happen
    pub unsafe fn new(ptr: *const T, finalizer: Option<extern "C" fn(Value)>) -> Opaque<'a, T> {
        let p = match finalizer {
            Some(f) => Value::alloc_custom(ptr, f),
            None => Value::alloc_custom(ptr, ignore),
        };

        Self::from_value(p)
    }

    /// Access the underlying pointer
    pub fn ptr(&self) -> *const T {
        self.0.custom_ptr_val()
    }

    /// Access the underlying mutable pointer
    pub fn ptr_mut(&mut self) -> *mut T {
        self.0.custom_ptr_val_mut()
    }
}

impl<'a, T> AsRef<T> for Opaque<'a, T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr() }
    }
}

impl<'a, T> AsMut<T> for Opaque<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr_mut() }
    }
}

/// `Array<A>` wraps an OCaml `'a array` without converting it to Rust, this introduces no
/// additional overhead compared to a `Value` type
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T: ToValue + FromValue>(Value, PhantomData<&'a T>);

unsafe impl<'a, T: ToValue + FromValue> ToValue for Array<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

unsafe impl<'a, T: ToValue + FromValue> FromValue for Array<'a, T> {
    fn from_value(value: Value) -> Self {
        Array(value, PhantomData)
    }
}

impl<'a> Array<'a, f64> {
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
    pub fn get_double(self, i: usize) -> Result<f64, Error> {
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
    pub unsafe fn get_double_unchecked(self, i: usize) -> f64 {
        *self.0.ptr_val::<f64>().add(i)
    }
}

impl<'a, T: ToValue + FromValue> Array<'a, T> {
    /// Allocate a new Array
    pub fn alloc(n: usize) -> Array<'a, T> {
        let x = crate::frame!((x) {
            x = unsafe { Value(alloc::caml_alloc(n, 0)) };
            x
        });
        Array(x, PhantomData)
    }

    /// Check if Array contains only doubles, if so `get_double` and `set_double` should be used
    /// to access values
    pub fn is_double_array(&self) -> bool {
        unsafe { alloc::caml_is_double_array((self.0).0) == 1 }
    }

    /// Array length
    pub fn len(&self) -> usize {
        unsafe { mlvalues::caml_array_length((self.0).0) }
    }

    /// Returns true when the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    /// Get array index (without bounds checking)
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, i: usize) -> T {
        T::from_value(self.0.field(i))
    }

    /// List as `Vec`
    pub fn to_vec(&self) -> Vec<T> {
        FromValue::from_value(self.to_value())
    }
}

/// `List<A>` wraps an OCaml `'a list` without converting it to Rust, this introduces no
/// additional overhead compared to a `Value` type
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct List<'a, T: ToValue + FromValue>(Value, PhantomData<&'a T>);

unsafe impl<'a, T: ToValue + FromValue> ToValue for List<'a, T> {
    fn to_value(&self) -> Value {
        self.0
    }
}

unsafe impl<'a, T: ToValue + FromValue> FromValue for List<'a, T> {
    fn from_value(value: Value) -> Self {
        List(value, PhantomData)
    }
}

impl<'a, T: ToValue + FromValue> List<'a, T> {
    /// An empty list
    #[inline(always)]
    pub fn empty() -> List<'a, T> {
        List(Value::unit(), PhantomData)
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

    /// Returns true when the list is empty
    pub fn is_empty(&self) -> bool {
        self.0 == Self::empty().0
    }

    /// Add an element to the front of the list
    pub fn push_hd(&mut self, v: T) {
        let tmp = crate::frame!((x, tmp) {
            x = self.0;
            unsafe {
                tmp = Value(crate::sys::alloc::caml_alloc_small(2, 0));
                tmp.store_field(0, v.to_value());
                tmp.store_field(1, x);
                tmp
            }
        });

        self.0 = tmp;
    }

    /// List head
    pub fn hd(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        Some(self.0.field(0))
    }

    /// List tail
    pub fn tl(&self) -> List<T> {
        if self.is_empty() {
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

/// `bigarray` contains wrappers for OCaml `Bigarray` values. These types can be used to transfer arrays of numbers between Rust
/// and OCaml directly without the allocation overhead of an `array` or `list`
pub mod bigarray {
    use super::*;
    use crate::sys::bigarray;

    /// Bigarray kind
    pub trait Kind {
        /// Array type
        type T: Clone + Copy;

        /// OCaml bigarray type identifier
        fn kind() -> i32;
    }

    macro_rules! make_kind {
        ($t:ty, $k:ident) => {
            impl Kind for $t {
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

    /// OCaml Bigarray.Array1 type, , this introduces no
    /// additional overhead compared to a `Value` type
    pub struct Array1<'a, T>(Value, PhantomData<&'a T>);

    unsafe impl<'a, T> crate::FromValue for Array1<'a, T> {
        fn from_value(value: Value) -> Array1<'a, T> {
            Array1(value, PhantomData)
        }
    }

    unsafe impl<'a, T> crate::ToValue for Array1<'a, T> {
        fn to_value(&self) -> Value {
            self.0
        }
    }

    impl<'a, T: 'a + Copy + Kind> From<&'a [T]> for Array1<'a, T> {
        fn from(x: &'a [T]) -> Array1<'a, T> {
            Array1::from_slice(x)
        }
    }

    impl<'a, T: 'a + Copy + Kind> From<Vec<T>> for Array1<'a, T> {
        fn from(x: Vec<T>) -> Array1<'a, T> {
            let mut arr = Array1::<T>::create(x.len());
            let data = arr.data_mut();
            data.copy_from_slice(x.as_slice());
            arr
        }
    }

    impl<'a, T: Copy + Kind> Array1<'a, T> {
        /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
        /// the `data` parameter must outlive the resulting bigarray or there is
        /// no guarantee the data will be valid. Use `Array1::from_slice` to clone the
        /// contents of a slice.
        pub fn of_slice(data: &'a mut [T]) -> Array1<'a, T> {
            let x = crate::frame!((x) {
                x = unsafe {
                    Value(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::EXTERNAL as i32,
                        1,
                        data.as_mut_ptr() as bigarray::Data,
                        data.len() as i32,
                    ))
                };
                x
            });
            Array1(x, PhantomData)
        }

        /// Convert from a slice to OCaml Bigarray, copying the array. This is the implemtation
        /// used by `Array1::from` for slices to avoid any potential lifetime issues
        pub fn from_slice(data: &'a [T]) -> Array1<'a, T> {
            Array1::from(data.to_vec())
        }

        /// Create a new OCaml `Bigarray.Array1` with the given type and size
        pub fn create(n: Size) -> Array1<'a, T> {
            let x = crate::frame!((x) {
                let data = unsafe { bigarray::malloc(n * mem::size_of::<T>()) };
                x = unsafe {
                    Value(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::MANAGED as i32,
                        1,
                        data,
                        n as mlvalues::Intnat,
                    ))
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
}
