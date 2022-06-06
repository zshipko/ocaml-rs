#![allow(unknown_lints)]
#![allow(clippy::derive_partial_eq_without_eq)]

use core::marker::PhantomData;

use crate::{Custom, FromValue, Raw, Runtime, ToValue, Value};

/// A handle to a Rust value/reference owned by the OCaml heap.
///
/// This should only be used with values allocated with `alloc_final` or `alloc_custom`,
/// for abstract pointers see `Value::alloc_abstract_ptr` and `Value::abstract_ptr_val`
#[derive(Clone, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
pub struct Pointer<T>(pub Value, PhantomData<T>);

unsafe impl<T> ToValue for Pointer<T> {
    fn to_value(&self, _rt: &Runtime) -> Value {
        self.0.clone()
    }
}

unsafe impl<T> FromValue for Pointer<T> {
    fn from_value(value: Value) -> Self {
        Pointer(value, PhantomData)
    }
}

impl<T: Custom> From<T> for Pointer<T> {
    fn from(x: T) -> Self {
        Pointer::alloc_custom(x)
    }
}

unsafe extern "C" fn drop_value<T>(raw: Raw) {
    let p = raw.as_pointer::<T>();
    p.drop_in_place();
}

impl<T: Custom> Pointer<T> {
    /// Allocate a `Custom` value
    pub fn alloc_custom(x: T) -> Pointer<T>
    where
        T: Custom,
    {
        unsafe {
            let mut ptr = Pointer(Value::alloc_custom::<T>(), PhantomData);
            ptr.set(x);
            ptr
        }
    }
}

impl<T> Pointer<T> {
    /// Allocate a new value with an optional custom finalizer and used/max
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Poiner::alloc_custom` when possible.
    pub fn alloc_final(
        x: T,
        finalizer: Option<unsafe extern "C" fn(Raw)>,
        used_max: Option<(usize, usize)>,
    ) -> Pointer<T> {
        unsafe {
            let value = match finalizer {
                Some(f) => Value::alloc_final::<T>(f, used_max),
                None => Value::alloc_final::<T>(drop_value::<T>, used_max),
            };
            let mut ptr = Pointer(value, PhantomData);
            ptr.set(x);
            ptr
        }
    }

    /// Allocate a new abstract value
    pub fn alloc(x: T) -> Pointer<T> {
        Self::alloc_final(x, None, None)
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
            core::ptr::write_unaligned(self.as_mut_ptr(), x); //core::ptr::read_unaligned(x));
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
