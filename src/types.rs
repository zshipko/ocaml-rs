#![allow(unknown_lints)]
#![allow(clippy::derive_partial_eq_without_eq)]

//! OCaml types represented in Rust, these are zero-copy and incur no additional overhead

use crate::{sys, CamlError, Error, Raw, Runtime, Tag};

use core::{
    iter::{IntoIterator, Iterator},
    marker::PhantomData,
    mem, slice,
};

use crate::value::{FromValue, Size, ToValue, Value};

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
/// Seq<A> wraps `'a Seq.t` without converting it to Rust
pub struct Seq<T: FromValue>(Value, PhantomData<T>);

unsafe impl<T: FromValue> ToValue for Seq<T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        self.0.clone()
    }
}

unsafe impl<T: FromValue> FromValue for Seq<T> {
    fn from_value(value: Value) -> Self {
        Seq(value, PhantomData)
    }
}

impl<T: FromValue> core::iter::Iterator for Seq<T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.0.seq_next() {
                Ok(Some((x, next))) => {
                    self.0 = next;
                    Some(Ok(T::from_value(x)))
                }
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        }
    }
}

/// `Array<A>` wraps an OCaml `'a array` without converting it to Rust
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Array<T: ToValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: ToValue + FromValue> ToValue for Array<T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        self.0.clone()
    }
}

unsafe impl<T: ToValue + FromValue> FromValue for Array<T> {
    fn from_value(value: Value) -> Self {
        Array(value, PhantomData)
    }
}

impl Array<crate::Float> {
    /// Set value to double array
    pub fn set_double(&mut self, i: usize, f: f64) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        if !self.is_f64() {
            return Err(Error::NotDoubleArray);
        }

        unsafe {
            self.set_f64_unchecked(i, f);
        };

        Ok(())
    }

    /// Set value to double array without bounds checking
    ///
    /// # Safety
    /// This function performs no bounds checking
    #[inline]
    pub unsafe fn set_f64_unchecked(&mut self, i: usize, f: f64) {
        self.0.store_double_field(i, f)
    }

    /// Get a value from a double array
    pub fn get_f64(&self, i: usize) -> Result<f64, Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }
        if !self.is_f64() {
            return Err(Error::NotDoubleArray);
        }

        Ok(unsafe { self.get_double_unchecked(i) })
    }

    /// Get a value from a double array without checking if the array is actually a double array
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn get_double_unchecked(&self, i: usize) -> f64 {
        self.0.double_field(i)
    }
}

impl<T: ToValue + FromValue> Array<T> {
    /// Allocate a new Array
    pub unsafe fn alloc(n: usize) -> Array<T> {
        let x = Value::alloc(n, Tag(0));
        Array(x, PhantomData)
    }

    /// Check if Array contains only doubles, if so `get_f64` and `set_f64` should be used
    /// to access values
    pub fn is_f64(&self) -> bool {
        unsafe { sys::caml_is_double_array(self.0.raw().0) == 1 }
    }

    /// Array length
    pub fn len(&self) -> usize {
        unsafe { sys::caml_array_length(self.0.raw().0) }
    }

    /// Returns true when the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set array index
    pub unsafe fn set(&mut self, rt: &Runtime, i: usize, v: &T) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        if self.is_f64() {
            self.0.store_double_field(i, v.to_value(rt).double_val())
        } else {
            self.set_unchecked(rt, i, v);
        }
        Ok(())
    }

    /// Set array index without bounds checking
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn set_unchecked(&mut self, rt: &Runtime, i: usize, v: &T) {
        self.0.store_field(rt, i, v);
    }

    /// Get array index
    pub unsafe fn get(&self, rt: &Runtime, i: usize) -> Result<T, Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        if self.is_f64() {
            return Ok(FromValue::from_value(self.0.double_field(i).to_value(rt)));
        }

        Ok(self.get_unchecked(i))
    }

    /// Get array index without bounds checking
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize) -> T {
        FromValue::from_value(self.0.field(i))
    }

    #[doc(hidden)]
    pub fn as_slice(&self) -> &[Raw] {
        unsafe { self.0.slice() }
    }

    #[doc(hidden)]
    pub fn as_mut_slice(&mut self) -> &mut [Raw] {
        unsafe { self.0.slice_mut() }
    }

    /// Array as `Vec`
    #[cfg(not(feature = "no-std"))]
    pub fn into_vec(self) -> Vec<T> {
        FromValue::from_value(self.0)
    }

    /// Array as `Vec`
    #[cfg(not(feature = "no-std"))]
    pub unsafe fn as_vec(&self, rt: &Runtime) -> Result<Vec<T>, Error> {
        let mut dest = Vec::new();
        let len = self.len();

        for i in 0..len {
            let x = self.get(rt, i)?;
            dest.push(x)
        }

        Ok(dest)
    }
}

/// `List<A>` wraps an OCaml `'a list` without converting it to Rust, this introduces no
/// additional overhead compared to a `Value` type
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct List<T: ToValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: ToValue + FromValue> ToValue for List<T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        self.0.clone()
    }
}

unsafe impl<T: ToValue + FromValue> FromValue for List<T> {
    fn from_value(value: Value) -> Self {
        List(value, PhantomData)
    }
}

impl<T: ToValue + FromValue> List<T> {
    /// An empty list
    #[inline(always)]
    pub fn empty() -> List<T> {
        List(Value::unit(), PhantomData)
    }

    /// Returns the number of items in `self`
    pub unsafe fn len(&self) -> usize {
        let mut length = 0;
        let mut tmp = self.0.raw();
        while tmp.0 != sys::EMPTY_LIST {
            let p = sys::field(tmp.0, 1);
            if p.is_null() {
                break;
            }
            tmp = (*p).into();
            length += 1;
        }
        length
    }

    /// Returns true when the list is empty
    pub fn is_empty(&self) -> bool {
        self.0 == Self::empty().0
    }

    /// Add an element to the front of the list returning the new list
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub unsafe fn add(self, rt: &Runtime, v: &T) -> List<T> {
        let item = v.to_value(rt);
        let mut dest = Value::alloc(2, Tag(0));
        dest.store_field(rt, 0, &item);
        dest.store_field(rt, 1, &self.0);
        List(dest, PhantomData)
    }

    /// List head
    pub fn hd(&self) -> Option<Value> {
        if self.is_empty() {
            return None;
        }

        unsafe { Some(self.0.field(0)) }
    }

    /// List tail
    pub fn tl(&self) -> List<T> {
        if self.is_empty() {
            return Self::empty();
        }

        unsafe { List(self.0.field(1), PhantomData) }
    }

    #[cfg(not(feature = "no-std"))]
    /// List as `Vec`
    pub fn into_vec(self) -> Vec<T> {
        self.into_iter().map(T::from_value).collect()
    }

    #[cfg(not(feature = "no-std"))]
    /// List as `LinkedList`
    pub fn into_linked_list(self) -> std::collections::LinkedList<T> {
        FromValue::from_value(self.0)
    }

    /// List iterator
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> ListIterator {
        ListIterator { inner: self.0 }
    }
}

impl<T: ToValue + FromValue> IntoIterator for List<T> {
    type Item = Value;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        List::into_iter(self)
    }
}

/// List iterator.
pub struct ListIterator {
    inner: Value,
}

impl Iterator for ListIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.raw().0 != sys::UNIT {
            unsafe {
                let val = self.inner.field(0);
                self.inner = self.inner.field(1);
                Some(val)
            }
        } else {
            None
        }
    }
}

/// `bigarray` contains wrappers for OCaml `Bigarray` values. These types can be used to transfer arrays of numbers between Rust
/// and OCaml directly without the allocation overhead of an `array` or `list`
pub mod bigarray {
    use super::*;
    use crate::sys::bigarray;

    /// Bigarray kind
    pub trait Kind {
        /// Array item type
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

    /// OCaml Bigarray.Array1 type, this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, PartialEq, Eq)]
    pub struct Array1<T>(Value, PhantomData<T>);

    unsafe impl<T> crate::FromValue for Array1<T> {
        fn from_value(value: Value) -> Array1<T> {
            Array1(value, PhantomData)
        }
    }

    unsafe impl<T> crate::ToValue for Array1<T> {
        fn to_value(&self, _rt: &Runtime) -> Value {
            self.0.clone()
        }
    }

    impl<T: Copy + Kind> Array1<T> {
        /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
        /// the `data` parameter must outlive the resulting bigarray or there is
        /// no guarantee the data will be valid. Use `Array1::from_slice` to clone the
        /// contents of a slice.
        pub unsafe fn of_slice(data: &mut [T]) -> Array1<T> {
            let x = Value::new(bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::EXTERNAL as i32,
                1,
                data.as_mut_ptr() as bigarray::Data,
                data.len() as sys::Intnat,
            ));
            Array1(x, PhantomData)
        }

        /// Convert from a slice to OCaml Bigarray, copying the array. This is the implemtation
        /// used by `Array1::from` for slices to avoid any potential lifetime issues
        #[cfg(not(feature = "no-std"))]
        pub unsafe fn from_slice(data: impl AsRef<[T]>) -> Array1<T> {
            let x = data.as_ref();
            let mut arr = Array1::<T>::create(x.len());
            let data = arr.data_mut();
            data.copy_from_slice(x);
            arr
        }

        /// Create a new OCaml `Bigarray.Array1` with the given type and size
        pub unsafe fn create(n: Size) -> Array1<T> {
            let data = { bigarray::malloc(n * mem::size_of::<T>()) };
            let x = Value::new(bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::EXTERNAL as i32,
                1,
                data as bigarray::Data,
                n as sys::Intnat,
            ));
            Array1(x, PhantomData)
        }

        /// Returns the number of items in `self`
        pub fn len(&self) -> Size {
            unsafe {
                let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
                let dim = slice::from_raw_parts((*ba).dim.as_ptr() as *const usize, 1);
                dim[0]
            }
        }

        /// Returns true when `self.len() == 0`
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Get underlying data as Rust slice
        pub fn data(&self) -> &[T] {
            unsafe {
                let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
                slice::from_raw_parts((*ba).data as *const T, self.len())
            }
        }

        /// Get underlying data as mutable Rust slice
        pub fn data_mut(&mut self) -> &mut [T] {
            unsafe {
                let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
                slice::from_raw_parts_mut((*ba).data as *mut T, self.len())
            }
        }
    }

    #[cfg(all(feature = "bigarray-ext", not(feature = "no-std")))]
    pub use super::bigarray_ext::*;
}

#[cfg(all(feature = "bigarray-ext", not(feature = "no-std")))]
pub(crate) mod bigarray_ext {
    use ndarray::{ArrayView2, ArrayView3, ArrayViewMut2, ArrayViewMut3, Dimension};

    use core::{marker::PhantomData, mem, ptr, slice};

    use crate::{
        bigarray::Kind,
        sys::{self, bigarray},
        FromValue, Runtime, ToValue, Value,
    };

    /// OCaml Bigarray.Array2 type, this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, PartialEq, Eq)]
    pub struct Array2<T>(Value, PhantomData<T>);

    impl<T: Copy + Kind> Array2<T> {
        /// Returns array view
        pub fn view(&self) -> ArrayView2<T> {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { ArrayView2::from_shape_ptr(self.shape(), (*ba).data as *const T) }
        }

        /// Returns mutable array view
        pub fn view_mut(&mut self) -> ArrayViewMut2<T> {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { ArrayViewMut2::from_shape_ptr(self.shape(), (*ba).data as *mut T) }
        }

        /// Returns the shape of `self`
        pub fn shape(&self) -> (usize, usize) {
            let dim = self.dim();
            (dim[0], dim[1])
        }

        /// Returns the number of items in `self`
        pub fn len(&self) -> usize {
            let dim = self.dim();
            dim[0] * dim[1]
        }

        /// Returns true when the list is empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn dim(&self) -> &[usize] {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { slice::from_raw_parts((*ba).dim.as_ptr() as *const usize, 2) }
        }
    }

    unsafe impl<T> FromValue for Array2<T> {
        fn from_value(value: Value) -> Array2<T> {
            Array2(value, PhantomData)
        }
    }

    unsafe impl<T> ToValue for Array2<T> {
        fn to_value(&self, _rt: &Runtime) -> Value {
            self.0.clone()
        }
    }

    impl<T: Copy + Kind> Array2<T> {
        /// Create a new OCaml `Bigarray.Array2` with the given type and shape
        pub unsafe fn create(dim: ndarray::Ix2) -> Array2<T> {
            let data = bigarray::malloc(dim.size() * mem::size_of::<T>());
            let x = Value::new(bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::EXTERNAL as i32,
                2,
                data as bigarray::Data,
                dim[0] as sys::Intnat,
                dim[1] as sys::Intnat,
            ));
            Array2(x, PhantomData)
        }

        /// Create Array2 from ndarray
        pub unsafe fn from_ndarray(data: ndarray::Array2<T>) -> Array2<T> {
            let dim = data.raw_dim();
            let array = Array2::create(dim);
            let ba = { array.0.custom_ptr_val::<bigarray::Bigarray>() };
            {
                ptr::copy_nonoverlapping(data.as_ptr(), (*ba).data as *mut T, dim.size());
            }
            array
        }
    }

    /// OCaml Bigarray.Array3 type, this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, PartialEq, Eq)]
    pub struct Array3<T>(Value, PhantomData<T>);

    impl<T: Copy + Kind> Array3<T> {
        /// Returns array view
        pub fn view(&self) -> ArrayView3<T> {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { ArrayView3::from_shape_ptr(self.shape(), (*ba).data as *const T) }
        }

        /// Returns mutable array view
        pub fn view_mut(&mut self) -> ArrayViewMut3<T> {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { ArrayViewMut3::from_shape_ptr(self.shape(), (*ba).data as *mut T) }
        }

        /// Returns the shape of `self`
        pub fn shape(&self) -> (usize, usize, usize) {
            let dim = self.dim();
            (dim[0], dim[1], dim[2])
        }

        /// Returns the number of items in `self`
        pub fn len(&self) -> usize {
            let dim = self.dim();
            dim[0] * dim[1] * dim[2]
        }

        /// Returns true when the list is empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn dim(&self) -> &[usize] {
            let ba = unsafe { self.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe { slice::from_raw_parts((*ba).dim.as_ptr() as *const usize, 3) }
        }
    }

    unsafe impl<T> FromValue for Array3<T> {
        fn from_value(value: Value) -> Array3<T> {
            Array3(value, PhantomData)
        }
    }

    unsafe impl<T> ToValue for Array3<T> {
        fn to_value(&self, _rt: &Runtime) -> Value {
            self.0.clone()
        }
    }

    impl<T: Copy + Kind> Array3<T> {
        /// Create a new OCaml `Bigarray.Array3` with the given type and shape
        pub unsafe fn create(dim: ndarray::Ix3) -> Array3<T> {
            let data = { bigarray::malloc(dim.size() * mem::size_of::<T>()) };
            let x = Value::new(bigarray::caml_ba_alloc_dims(
                T::kind() | bigarray::Managed::MANAGED as i32,
                3,
                data,
                dim[0] as sys::Intnat,
                dim[1] as sys::Intnat,
                dim[2] as sys::Intnat,
            ));
            Array3(x, PhantomData)
        }

        /// Create Array3 from ndarray
        pub unsafe fn from_ndarray(data: ndarray::Array3<T>) -> Array3<T> {
            let dim = data.raw_dim();
            let array = Array3::create(dim);
            let ba = { array.0.custom_ptr_val::<bigarray::Bigarray>() };
            {
                ptr::copy_nonoverlapping(data.as_ptr(), (*ba).data as *mut T, dim.size());
            }
            array
        }
    }
}
