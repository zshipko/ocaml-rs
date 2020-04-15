//! OCaml types represented in Rust, these are zero-copy and incur no additional overhead

use crate::{sys, CamlError, Error};

use std::marker::PhantomData;
use std::{mem, slice};

use crate::value::{FromValue, Size, ToValue, Value};

/// A handle to a Rust value/reference owned by the OCaml heap
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Pointer<T>(pub Value, PhantomData<T>);

unsafe impl<T> ToValue for Pointer<T> {
    fn to_value(self) -> Value {
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
        x: T,
        finalizer: Option<unsafe extern "C" fn(Value)>,
        used_max: Option<(usize, usize)>,
    ) -> Pointer<T> {
        let mut ptr = Pointer::from_value(match finalizer {
            Some(f) => Value::alloc_final::<T>(f, used_max),
            None => Value::alloc_final::<T>(ignore, used_max),
        });
        ptr.set(x);
        ptr
    }

    /// Allocate a `Custom` value
    pub fn alloc_custom(x: T) -> Pointer<T>
    where
        T: crate::Custom,
    {
        let mut ptr = Pointer::from_value(Value::alloc_custom::<T>());
        ptr.set(x);
        ptr
    }

    /// Drop pointer in place
    ///
    /// # Safety
    /// This should only be used when you're in control of the underlying value and want to drop
    /// it. It should only be called once.
    pub unsafe fn drop_in_place(mut self) {
        std::ptr::drop_in_place(self.as_mut_ptr())
    }

    /// Replace the inner value with the provided argument
    pub fn set(&mut self, x: T) {
        unsafe {
            std::ptr::write_unaligned(self.as_mut_ptr(), x);
        }
    }

    /// Access the underlying pointer
    pub fn as_ptr(&self) -> *const T {
        self.0.custom_ptr_val()
    }

    /// Access the underlying mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.custom_mut_ptr_val()
    }
}

impl<T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }
}

impl<T> AsMut<T> for Pointer<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

/// `Array<A>` wraps an OCaml `'a array` without converting it to Rust
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Array<T: ToValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: ToValue + FromValue> ToValue for Array<T> {
    fn to_value(self) -> Value {
        self.0
    }
}

unsafe impl<T: ToValue + FromValue> FromValue for Array<T> {
    fn from_value(value: Value) -> Self {
        Array(value, PhantomData)
    }
}

impl<'a> Array<f64> {
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
        let ptr = self.0.ptr_val::<f64>().add(i) as *mut f64;
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
        *self.0.ptr_val::<f64>().add(i)
    }
}

impl<T: ToValue + FromValue> Array<T> {
    /// Allocate a new Array
    pub fn alloc(n: usize) -> Array<T> {
        let x = crate::frame!((x) {
            x = unsafe { Value(sys::caml_alloc(n, 0)) };
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
    pub fn set(&mut self, i: usize, v: T) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }
        unsafe { self.set_unchecked(i, v) }
        Ok(())
    }

    /// Set array index without bounds checking
    ///
    /// # Safety
    ///
    /// This function does not perform bounds checking
    #[inline]
    pub unsafe fn set_unchecked(&mut self, i: usize, v: T) {
        self.0.store_field(i, v);
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
    pub fn to_vec(&self) -> Vec<T> {
        FromValue::from_value(self.0)
    }
}

/// `List<A>` wraps an OCaml `'a list` without converting it to Rust, this introduces no
/// additional overhead compared to a `Value` type
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct List<T: ToValue + FromValue>(Value, PhantomData<T>);

unsafe impl<T: ToValue + FromValue> ToValue for List<T> {
    fn to_value(self) -> Value {
        self.0
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
    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut tmp = self.0;
        while tmp.0 != sys::EMPTY_LIST {
            tmp = tmp.field(1);
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
    pub fn add(self, v: T) -> List<T> {
        local!(x, tmp);
        x = v.to_value();
        unsafe {
            tmp = Value(sys::caml_alloc(2, 0));
            tmp.store_field(0, x);
            tmp.store_field(1, self.0);
        }
        List(tmp, PhantomData)
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

    /// List as `LinkedList`
    pub fn to_linked_list(&self) -> std::collections::LinkedList<T> {
        FromValue::from_value(self.0)
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

    /// OCaml Bigarray.Array1 type, , this introduces no
    /// additional overhead compared to a `Value` type
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq)]
    pub struct Array1<T>(Value, PhantomData<T>);

    unsafe impl<T> crate::FromValue for Array1<T> {
        fn from_value(value: Value) -> Array1<T> {
            Array1(value, PhantomData)
        }
    }

    unsafe impl<T> crate::ToValue for Array1<T> {
        fn to_value(self) -> Value {
            self.0
        }
    }

    impl<T: Copy + Kind> From<&'static [T]> for Array1<T> {
        fn from(x: &'static [T]) -> Array1<T> {
            Array1::from_slice(x)
        }
    }

    impl<T: Copy + Kind> From<Vec<T>> for Array1<T> {
        fn from(x: Vec<T>) -> Array1<T> {
            let mut arr = Array1::<T>::create(x.len());
            let data = arr.data_mut();
            data.copy_from_slice(x.as_slice());
            arr
        }
    }

    impl<T: Copy + Kind> Array1<T> {
        /// Array1::of_slice is used to convert from a slice to OCaml Bigarray,
        /// the `data` parameter must outlive the resulting bigarray or there is
        /// no guarantee the data will be valid. Use `Array1::from_slice` to clone the
        /// contents of a slice.
        pub fn of_slice(data: &mut [T]) -> Array1<T> {
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
        pub fn from_slice(data: &[T]) -> Array1<T> {
            Array1::from(data.to_vec())
        }

        /// Create a new OCaml `Bigarray.Array1` with the given type and size
        pub fn create(n: Size) -> Array1<T> {
            let x = crate::frame!((x) {
                let data = unsafe { bigarray::malloc(n * mem::size_of::<T>()) };
                x = unsafe {
                    Value(bigarray::caml_ba_alloc_dims(
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
            let ba = self.0.custom_ptr_val::<bigarray::Bigarray>();
            unsafe { (*ba).dim as usize }
        }

        /// Returns true when `self.len() == 0`
        pub fn is_empty(self) -> bool {
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
