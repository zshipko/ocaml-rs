//! OCaml types represented in Rust, these are zero-copy and incur no additional overhead

use crate::{sys, CamlError, Error, Runtime};

use core::{
    iter::{IntoIterator, Iterator},
    marker::PhantomData,
    mem, slice,
};

use crate::value::{FromValue, IntoValue, Size, Value};

/// A handle to a Rust value/reference owned by the OCaml heap.
///
/// This should only be used with values allocated with `alloc_final` or `alloc_custom`,
/// for abstract pointers see `Value::alloc_abstract_ptr` and `Value::abstract_ptr_val`
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Pointer<T>(pub Value, PhantomData<T>);

unsafe impl<T> IntoValue for Pointer<T> {
    fn into_value(self, _rt: &Runtime) -> Value {
        self.0
    }
}

unsafe impl<T> FromValue for Pointer<T> {
    fn from_value(value: Value) -> Self {
        Pointer(value, PhantomData)
    }
}

unsafe extern "C" fn ignore(_: Value) {}

impl<T> Pointer<T> {
    /// Allocate a new value with an optional custom finalizer and used/max
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Poiner::alloc_custom` when possible.
    pub fn alloc_final(
        rt: &Runtime,
        x: T,
        finalizer: Option<unsafe extern "C" fn(Value)>,
        used_max: Option<(usize, usize)>,
    ) -> Pointer<T> {
        unsafe {
            let value = match finalizer {
                Some(f) => Value::alloc_final::<T>(rt, f, used_max),
                None => Value::alloc_final::<T>(rt, ignore, used_max),
            };
            let mut ptr = Pointer::from_value(value);
            ptr.set(x);
            ptr
        }
    }

    /// Allocate a `Custom` value
    pub fn alloc_custom(rt: &Runtime, x: T) -> Pointer<T>
    where
        T: crate::Custom,
    {
        unsafe {
            let mut ptr = Pointer::from_value(Value::alloc_custom::<T>(rt));
            ptr.set(x);
            ptr
        }
    }

    /// Drop pointer in place
    ///
    /// # Safety
    /// This should only be used when you're in control of the underlying value and want to drop
    /// it. It should only be called once.
    pub unsafe fn drop_in_place(mut self) {
        core::ptr::drop_in_place(self.as_mut_ptr())
    }

    /// Replace the inner value with the provided argument
    pub fn set(&mut self, x: T) {
        unsafe {
            core::ptr::write_unaligned(self.as_mut_ptr(), x);
        }
    }

    /// Access the underlying pointer
    pub fn as_ptr(&self) -> *const T {
        unsafe { self.0.custom_ptr_val() }
    }

    /// Access the underlying mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut T {
        unsafe { self.0.custom_ptr_val_mut() }
    }
}

impl<'a, T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }
}

impl<'a, T> AsMut<T> for Pointer<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

/// `Array<A>` wraps an OCaml `'a array` without converting it to Rust
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Array<T: IntoValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: IntoValue + FromValue> IntoValue for Array<T> {
    fn into_value(self, _rt: &Runtime) -> Value {
        self.0
    }
}

unsafe impl<T: IntoValue + FromValue> FromValue for Array<T> {
    fn from_value(value: Value) -> Self {
        Array(value, PhantomData)
    }
}

impl Array<f64> {
    /// Set value to double array
    pub fn set_double(&mut self, i: usize, f: f64) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        if !self.is_double_array() {
            return Err(Error::NotDoubleArray);
        }

        unsafe {
            self.set_double_unchecked(i, f);
        };

        Ok(())
    }

    /// Set value to double array without bounds checking
    ///
    /// # Safety
    /// This function performs no bounds checking
    #[inline]
    pub unsafe fn set_double_unchecked(&mut self, i: usize, f: f64) {
        let ptr = ((self.0).0 as *mut f64).add(i);
        *ptr = f;
    }

    /// Get a value from a double array
    pub fn get_double(self, i: usize) -> Result<f64, Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }
        if !self.is_double_array() {
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
    pub unsafe fn get_double_unchecked(self, i: usize) -> f64 {
        *((self.0).0 as *mut f64).add(i)
    }
}

impl<T: IntoValue + FromValue> Array<T> {
    /// Allocate a new Array
    pub fn alloc(rt: &Runtime, n: usize) -> Array<T> {
        let x = crate::frame!(rt: (x) {
            x = unsafe { Value::new(sys::caml_alloc(n, 0)) };
            x
        });
        Array(x, PhantomData)
    }

    /// Check if Array contains only doubles, if so `get_double` and `set_double` should be used
    /// to access values
    pub fn is_double_array(&self) -> bool {
        unsafe { sys::caml_is_double_array((self.0).0) == 1 }
    }

    /// Array length
    pub fn len(&self) -> usize {
        unsafe { sys::caml_array_length((self.0).0) }
    }

    /// Returns true when the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set array index
    pub fn set(&mut self, rt: &Runtime, i: usize, v: T) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }
        unsafe { self.set_unchecked(rt, i, v) }
        Ok(())
    }

    /// Set array index without bounds checking
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn set_unchecked(&mut self, rt: &Runtime, i: usize, v: T) {
        self.0.store_field(rt, i, v);
    }

    /// Get array index
    pub fn get(&self, i: usize) -> Result<T, Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }
        Ok(unsafe { self.get_unchecked(i) })
    }

    /// Get array index without bounds checking
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize) -> T {
        T::from_value(self.0.field(i))
    }

    /// Array as slice
    pub fn as_slice(&self) -> &[Value] {
        FromValue::from_value(self.0)
    }

    /// Array as mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [Value] {
        FromValue::from_value(self.0)
    }

    /// Array as `Vec`
    #[cfg(not(feature = "no-std"))]
    pub fn to_vec(&self) -> Vec<T> {
        FromValue::from_value(self.0)
    }
}

/// `List<A>` wraps an OCaml `'a list` without converting it to Rust, this introduces no
/// additional overhead compared to a `Value` type
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct List<T: IntoValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: IntoValue + FromValue> IntoValue for List<T> {
    fn into_value(self, _rt: &Runtime) -> Value {
        self.0
    }
}

unsafe impl<'a, T: IntoValue + FromValue> FromValue for List<T> {
    fn from_value(value: Value) -> Self {
        List(value, PhantomData)
    }
}

impl<'a, T: IntoValue + FromValue> List<T> {
    /// An empty list
    #[inline(always)]
    pub fn empty() -> List<T> {
        List(Value::unit(), PhantomData)
    }

    /// Returns the number of items in `self`
    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut tmp = self.0;
        while tmp.0 != sys::EMPTY_LIST {
            tmp = unsafe { tmp.field(1) };
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
    pub fn add(self, rt: &'a Runtime, v: T) -> List<T> {
        frame!(rt: (x, tmp) {
                x = v.into_value(rt);
            unsafe {
                tmp = Value::new(sys::caml_alloc(2, 0));
                tmp.store_field(rt, 0, x);
                tmp.store_field(rt, 1, self.0);
            }
            List(tmp, PhantomData)
        })
    }

    /// List head
    pub fn hd(&self) -> Option<T> {
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

        unsafe { self.0.field(1) }
    }

    #[cfg(not(feature = "no-std"))]
    /// List as `Vec`
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }

    #[cfg(not(feature = "no-std"))]
    /// List as `LinkedList`
    pub fn to_linked_list(&self) -> std::collections::LinkedList<T> {
        FromValue::from_value(self.0)
    }

    /// List iterator
    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            inner: self.0,
            _marker: PhantomData,
        }
    }
}

impl<T: IntoValue + FromValue> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// List iterator.
pub struct ListIterator<T: IntoValue + FromValue> {
    inner: Value,
    _marker: PhantomData<T>,
}

impl<'a, T: IntoValue + FromValue> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner != Value::unit() {
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
    #[derive(Clone, Copy, PartialEq)]
    pub struct Array1<T>(Value, PhantomData<T>);

    unsafe impl<T> crate::FromValue for Array1<T> {
        fn from_value(value: Value) -> Array1<T> {
            unsafe { Array1(Value::new(value.0), PhantomData) }
        }
    }

    unsafe impl<T> crate::IntoValue for Array1<T> {
        fn into_value(self, _rt: &Runtime) -> Value {
            self.0
        }
    }

    impl<T: Copy + Kind> Array1<T> {
        /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
        /// the `data` parameter must outlive the resulting bigarray or there is
        /// no guarantee the data will be valid. Use `Array1::from_slice` to clone the
        /// contents of a slice.
        pub fn of_slice(rt: &Runtime, data: &mut [T]) -> Array1<T> {
            let x = crate::frame!(rt: (x) {
                x = unsafe {
                    Value::new(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::EXTERNAL as i32,
                        1,
                        data.as_mut_ptr() as bigarray::Data,
                        data.len() as sys::Intnat,
                    ))
                };
                x
            });
            Array1(x, PhantomData)
        }

        /// Convert from a slice to OCaml Bigarray, copying the array. This is the implemtation
        /// used by `Array1::from` for slices to avoid any potential lifetime issues
        #[cfg(not(feature = "no-std"))]
        pub fn from_slice(rt: &Runtime, data: impl AsRef<[T]>) -> Array1<T> {
            let x = data.as_ref();
            let mut arr = Array1::<T>::create(rt, x.len());
            let data = arr.data_mut();
            data.copy_from_slice(x);
            arr
        }

        /// Create a new OCaml `Bigarray.Array1` with the given type and size
        pub fn create(rt: &Runtime, n: Size) -> Array1<T> {
            let x = crate::frame!(rt: (x) {
                let data = unsafe { bigarray::malloc(n * mem::size_of::<T>()) };
                x = unsafe {
                    Value::new(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::MANAGED as i32,
                        1,
                        data,
                        n as sys::Intnat,
                    ))
                };
                x
            });
            Array1(x, PhantomData)
        }

        /// Returns the number of items in `self`
        pub fn len(self) -> Size {
            unsafe {
                let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
                let dim = slice::from_raw_parts((*ba).dim.as_ptr() as *const usize, 1);
                dim[0]
            }
        }

        /// Returns true when `self.len() == 0`
        pub fn is_empty(self) -> bool {
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
        FromValue, IntoValue, Runtime, Value,
    };

    /// OCaml Bigarray.Array2 type, this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq)]
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
            unsafe { Array2(Value::new(value.0), PhantomData) }
        }
    }

    unsafe impl<T> IntoValue for Array2<T> {
        fn into_value(self, _rt: &Runtime) -> Value {
            self.0
        }
    }

    impl<T: Copy + Kind> Array2<T> {
        /// Create a new OCaml `Bigarray.Array2` with the given type and shape
        pub fn create(rt: &Runtime, dim: ndarray::Ix2) -> Array2<T> {
            let x = crate::frame!(rt: (x) {
                let data = unsafe { bigarray::malloc(dim.size() * mem::size_of::<T>()) };
                x = unsafe {
                    Value::new(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::MANAGED as i32,
                        2,
                        data,
                        dim[0] as sys::Intnat,
                        dim[1] as sys::Intnat,
                    ))
                };
                x
            });
            Array2(x, PhantomData)
        }

        /// Create Array2 from ndarray
        pub fn from_ndarray(rt: &Runtime, data: ndarray::Array2<T>) -> Array2<T> {
            let dim = data.raw_dim();
            let array = Array2::create(rt, dim);
            let ba = unsafe { array.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), (*ba).data as *mut T, dim.size());
            }
            array
        }
    }

    /// OCaml Bigarray.Array3 type, this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq)]
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
            unsafe { Array3(Value::new(value.0), PhantomData) }
        }
    }

    unsafe impl<T> IntoValue for Array3<T> {
        fn into_value(self, _rt: &Runtime) -> Value {
            self.0
        }
    }

    impl<T: Copy + Kind> Array3<T> {
        /// Create a new OCaml `Bigarray.Array3` with the given type and shape
        pub fn create(rt: &Runtime, dim: ndarray::Ix3) -> Array3<T> {
            let x = crate::frame!(rt: (x) {
                let data = unsafe { bigarray::malloc(dim.size() * mem::size_of::<T>()) };
                x = unsafe {
                    Value::new(bigarray::caml_ba_alloc_dims(
                        T::kind() | bigarray::Managed::MANAGED as i32,
                        3,
                        data,
                        dim[0] as sys::Intnat,
                        dim[1] as sys::Intnat,
                        dim[2] as sys::Intnat,
                    ))
                };
                x
            });
            Array3(x, PhantomData)
        }

        /// Create Array3 from ndarray
        pub fn from_ndarray(rt: &Runtime, data: ndarray::Array3<T>) -> Array3<T> {
            let dim = data.raw_dim();
            let array = Array3::create(rt, dim);
            let ba = unsafe { array.0.custom_ptr_val::<bigarray::Bigarray>() };
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), (*ba).data as *mut T, dim.size());
            }
            array
        }
    }
}
