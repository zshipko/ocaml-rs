use core::convert::TryInto;

use crate::{
    sys,
    value::{FromValue, ToValue, Value},
    Raw, Runtime, Tag,
};

unsafe impl<T: ToValue> ToValue for &T {
    fn to_value(&self, rt: &Runtime) -> Value {
        ToValue::to_value(*self, rt)
    }
}

macro_rules! value_i {
    ($t:ty) => {
        unsafe impl ToValue for $t {
            fn to_value(&self, _rt: &Runtime) -> $crate::Value {
                unsafe { $crate::Value::int(*self as crate::Int) }
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                unsafe { v.int_val() as $t }
            }
        }
    };
    ($($t:ty),*) => {
        $(value_i!($t);)*
    }
}

macro_rules! value_f {
    ($t:ty) => {
        unsafe impl ToValue for $t {
            fn to_value(&self, _rt: &Runtime) -> $crate::Value {
                unsafe { $crate::Value::double(*self as crate::Float) }
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                unsafe { v.double_val () as $t }
            }
        }
    };
    ($($t:ty),*) => {
        $(value_f!($t);)*
    }
}

value_i!(i8, u8, i16, u16, crate::Int, crate::Uint);
value_f!(f32, f64);

unsafe impl ToValue for i64 {
    fn to_value(&self, _rt: &Runtime) -> crate::Value {
        unsafe { Value::int64(*self) }
    }
}

unsafe impl FromValue for i64 {
    fn from_value(v: Value) -> i64 {
        unsafe { v.int64_val() }
    }
}

unsafe impl ToValue for u64 {
    fn to_value(&self, _rt: &Runtime) -> crate::Value {
        unsafe { Value::int64(*self as i64) }
    }
}

unsafe impl FromValue for u64 {
    fn from_value(v: Value) -> u64 {
        unsafe { v.int64_val() as u64 }
    }
}

unsafe impl ToValue for i32 {
    fn to_value(&self, _rt: &Runtime) -> crate::Value {
        unsafe { Value::int32(*self) }
    }
}

unsafe impl FromValue for i32 {
    fn from_value(v: Value) -> i32 {
        unsafe { v.int32_val() }
    }
}

unsafe impl ToValue for u32 {
    fn to_value(&self, _rt: &Runtime) -> crate::Value {
        unsafe { Value::int32(*self as i32) }
    }
}

unsafe impl FromValue for u32 {
    fn from_value(v: Value) -> u32 {
        unsafe { v.int32_val() as u32 }
    }
}

struct Incr(usize);

impl Incr {
    fn get(&mut self) -> usize {
        let i = self.0;
        self.0 = i + 1;
        i
    }
}

macro_rules! tuple_impl {
    ($($t:ident: $n:tt),*) => {
        unsafe impl<$($t: FromValue),*> FromValue for ($($t,)*) {
            fn from_value(v: Value) -> ($($t,)*) {
                let mut i = Incr(0);
                #[allow(unused)]
                (
                    $(
                        $t::from_value(unsafe { v.field(i.get()) }),
                    )*
                )
            }
        }

        unsafe impl<$($t: ToValue),*> ToValue for ($($t,)*) {
            fn to_value(&self, rt: &Runtime) -> crate::Value {
                #[allow(unused)]
                let mut len = 0;
                $(
                    #[allow(unused)]
                    {
                        len = $n + 1;
                    }
                )*

                unsafe {
                    let mut v = $crate::Value::alloc(len, Tag(0));
                    $(
                        v.store_field(rt, $n, &self.$n);
                    )*

                    v
                }
            }
        }
    };
}

tuple_impl!(A: 0);
tuple_impl!(A: 0, B: 1);
tuple_impl!(A: 0, B: 1, C: 2);
tuple_impl!(A: 0, B: 1, C: 2, D: 3);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18, T: 19);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18, T: 19, U: 20);

unsafe impl ToValue for bool {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::int(*self as isize) }
    }
}

unsafe impl FromValue for bool {
    fn from_value(v: Value) -> bool {
        unsafe { v.int_val() != 0 }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl ToValue for String {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::string(self.as_str()) }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl FromValue for String {
    fn from_value(value: Value) -> String {
        unsafe { value.string_val().into() }
    }
}

unsafe impl ToValue for () {
    fn to_value(&self, _rt: &Runtime) -> Value {
        Value::unit()
    }
}

unsafe impl FromValue for () {
    fn from_value(_value: Value) {}
}

unsafe impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        if value.raw().0 == sys::NONE {
            return None;
        }

        unsafe { Some(T::from_value(value.field(0))) }
    }
}

unsafe impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self, rt: &Runtime) -> Value {
        match self {
            Some(y) => unsafe { Value::some(rt, y) },
            None => Value::none(),
        }
    }
}

unsafe impl<'a> FromValue for &'a str {
    fn from_value(value: Value) -> &'a str {
        unsafe {
            let len = sys::caml_string_length(value.raw().0);
            let ptr = sys::string_val(value.raw().0);
            let slice = ::core::slice::from_raw_parts(ptr, len);
            ::core::str::from_utf8(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl ToValue for &str {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::string(self) }
    }
}

unsafe impl<'a> FromValue for &'a mut str {
    fn from_value(value: Value) -> &'a mut str {
        unsafe {
            let len = sys::caml_string_length(value.raw().0);
            let ptr = sys::string_val(value.raw().0);
            let slice = ::core::slice::from_raw_parts_mut(ptr, len);
            ::core::str::from_utf8_mut(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl ToValue for &mut str {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::string(self) }
    }
}

unsafe impl<'a> FromValue for &'a [u8] {
    fn from_value(value: Value) -> &'a [u8] {
        unsafe {
            let len = sys::caml_string_length(value.raw().0);
            let ptr = sys::string_val(value.raw().0);
            ::core::slice::from_raw_parts(ptr, len)
        }
    }
}

unsafe impl ToValue for &[u8] {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::bytes(self) }
    }
}

unsafe impl<'a> FromValue for &'a mut [u8] {
    fn from_value(value: Value) -> &'a mut [u8] {
        unsafe {
            let len = sys::caml_string_length(value.raw().0);
            let ptr = sys::string_val(value.raw().0);
            ::core::slice::from_raw_parts_mut(ptr, len)
        }
    }
}

unsafe impl ToValue for &mut [u8] {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::bytes(self) }
    }
}

unsafe impl<const N: usize> FromValue for [u8; N] {
    fn from_value(value: Value) -> Self {
        unsafe {
            let len = sys::caml_string_length(value.raw().0);
            assert!(len == N);
            let ptr = sys::string_val(value.raw().0);
            ::core::slice::from_raw_parts(ptr, len).try_into().unwrap()
        }
    }
}

unsafe impl<const N: usize> ToValue for [u8; N] {
    fn to_value(&self, _rt: &Runtime) -> Value {
        unsafe { Value::bytes(self) }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: FromValue> FromValue for Box<V> {
    fn from_value(v: Value) -> Box<V> {
        Box::new(V::from_value(v))
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: ToValue> ToValue for Box<V> {
    fn to_value(&self, rt: &Runtime) -> Value {
        (**self).to_value(rt)
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: 'static + ToValue> ToValue for Vec<V> {
    fn to_value(&self, rt: &Runtime) -> Value {
        let len = self.len();

        if core::any::TypeId::of::<f64>() == core::any::TypeId::of::<V>() && sys::FLAT_FLOAT_ARRAY {
            let mut arr = unsafe { Value::alloc_double_array(len) };
            for (i, v) in self.iter().enumerate() {
                unsafe {
                    arr.store_double_field(i, v.to_value(rt).double_val());
                }
            }
            arr
        } else {
            let mut arr = unsafe { Value::alloc(len, 0.into()) };
            for (i, v) in self.iter().enumerate() {
                unsafe {
                    arr.store_field(rt, i, v);
                }
            }
            arr
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: FromValue> FromValue for Vec<V> {
    fn from_value(v: Value) -> Vec<V> {
        unsafe {
            let len = crate::sys::caml_array_length(v.raw().0);
            let is_double = sys::caml_is_double_array(v.raw().0) == 1 && sys::FLAT_FLOAT_ARRAY;
            let mut dst = Vec::with_capacity(len);
            if is_double {
                let mut tmp = Value::double(0.0);
                for i in 0..len {
                    tmp.store_double_val(v.double_field(i));
                    dst.push(V::from_value(Value::new(tmp.raw().0)));
                }
            } else {
                for i in 0..len {
                    dst.push(V::from_value(Value::new(*crate::sys::field(v.raw().0, i))))
                }
            }
            dst
        }
    }
}

unsafe impl<'a> FromValue for &'a [Raw] {
    fn from_value(value: Value) -> &'a [Raw] {
        unsafe {
            ::core::slice::from_raw_parts(
                crate::sys::field(value.raw().0, 0) as *mut Raw,
                crate::sys::wosize_val(value.raw().0),
            )
        }
    }
}

unsafe impl<'a> FromValue for &'a mut [Raw] {
    fn from_value(value: Value) -> &'a mut [Raw] {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                crate::sys::field(value.raw().0, 0) as *mut Raw,
                crate::sys::wosize_val(value.raw().0),
            )
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: Ord + FromValue, V: FromValue> FromValue for std::collections::BTreeMap<K, V> {
    fn from_value(v: Value) -> std::collections::BTreeMap<K, V> {
        let mut dest = std::collections::BTreeMap::new();
        unsafe {
            let mut tmp = v;
            while tmp.raw().0 != crate::sys::EMPTY_LIST {
                let (k, v) = FromValue::from_value(tmp.field(0));
                dest.insert(k, v);
                tmp = tmp.field(1);
            }
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: ToValue, V: ToValue> ToValue for std::collections::BTreeMap<K, V> {
    fn to_value(&self, rt: &Runtime) -> Value {
        let mut list = crate::List::empty();

        for (k, v) in self.iter().rev() {
            let k_ = k.to_value(rt);
            let v_ = v.to_value(rt);
            list = unsafe { list.add(rt, &(k_, v_)) };
        }

        list.to_value(rt)
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: FromValue> FromValue for std::collections::LinkedList<T> {
    fn from_value(v: Value) -> std::collections::LinkedList<T> {
        let mut dest: std::collections::LinkedList<T> = std::collections::LinkedList::new();

        unsafe {
            let mut tmp = v;
            while tmp.raw().0 != crate::sys::EMPTY_LIST {
                let t = T::from_value(tmp.field(0));
                dest.push_back(t);
                tmp = tmp.field(1);
            }
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: ToValue> ToValue for std::collections::LinkedList<T> {
    fn to_value(&self, rt: &Runtime) -> Value {
        let mut list = crate::List::empty();

        for v in self.iter().rev() {
            let v_ = v.to_value(rt);
            list = unsafe { list.add(rt, &v_) };
        }

        list.to_value(rt)
    }
}
