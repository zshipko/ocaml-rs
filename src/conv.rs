use crate::{
    value::{FromValue, ToValue, Value},
    Tag,
};

macro_rules! value_i {
    ($t:ty) => {
        unsafe impl ToValue for $t {
            fn to_value(self) -> $crate::Value {
                $crate::Value::int(self as crate::Int)
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                v.int_val() as $t
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
            fn to_value(self) -> $crate::Value {
                $crate::Value::float(self as crate::Float)
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                v.float_val() as $t
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
    fn to_value(self) -> crate::Value {
        Value::int64(self)
    }
}

unsafe impl FromValue for i64 {
    fn from_value(v: crate::Value) -> i64 {
        v.int64_val()
    }
}

unsafe impl ToValue for u64 {
    fn to_value(self) -> crate::Value {
        Value::int64(self as i64)
    }
}

unsafe impl FromValue for u64 {
    fn from_value(v: crate::Value) -> u64 {
        v.int64_val() as u64
    }
}

unsafe impl ToValue for i32 {
    fn to_value(self) -> crate::Value {
        Value::int32(self)
    }
}

unsafe impl FromValue for i32 {
    fn from_value(v: crate::Value) -> i32 {
        v.int32_val()
    }
}

unsafe impl ToValue for u32 {
    fn to_value(self) -> crate::Value {
        Value::int64(self as i64)
    }
}

unsafe impl FromValue for u32 {
    fn from_value(v: crate::Value) -> u32 {
        v.int32_val() as u32
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
            fn from_value(v: crate::Value) -> ($($t,)*) {
                let mut i = Incr(0);
                #[allow(unused)]
                (
                    $(
                        $t::from_value(v.field(i.get())),
                    )*
                )
            }
        }

        unsafe impl<$($t: ToValue),*> ToValue for ($($t,)*) {
            fn to_value(self) -> crate::Value {
                #[allow(unused)]
                let mut len = 0;
                $(
                    #[allow(unused)]
                    {
                        len = $n;
                    }
                )*

                crate::local!(v, x);
                v = $crate::Value::alloc(len, Tag(0));
                $(
                    x = $t::to_value(self.$n);
                    v.store_field($n, x);
                )*
                v
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
    fn to_value(self) -> Value {
        Value::int(self as isize)
    }
}

unsafe impl FromValue for bool {
    fn from_value(v: Value) -> bool {
        v.int_val() != 0
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl ToValue for String {
    fn to_value(self) -> Value {
        unsafe {
            let value = crate::sys::caml_alloc_string(self.len());
            let ptr = crate::sys::string_val(value);
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl FromValue for String {
    fn from_value(value: Value) -> String {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, len);
            ::std::str::from_utf8(slice).expect("Invalid UTF-8").into()
        }
    }
}

unsafe impl ToValue for () {
    fn to_value(self) -> Value {
        Value::unit()
    }
}

unsafe impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        if value == Value::none() {
            return None;
        }

        Some(value.field(0))
    }
}

unsafe impl<T: ToValue> ToValue for Option<T> {
    fn to_value(self) -> Value {
        match self {
            Some(x) => Value::some(x.to_value()),
            None => Value::none(),
        }
    }
}

unsafe impl FromValue for &str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe {
            let slice = ::core::slice::from_raw_parts(ptr, len);
            ::core::str::from_utf8(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl ToValue for &str {
    fn to_value(self) -> Value {
        unsafe {
            let value = crate::sys::caml_alloc_string(self.len());
            let ptr = crate::sys::string_val(value);
            core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

unsafe impl FromValue for &mut str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe {
            let slice = ::core::slice::from_raw_parts_mut(ptr, len);
            ::core::str::from_utf8_mut(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl ToValue for &mut str {
    fn to_value(self) -> Value {
        unsafe {
            let value = crate::sys::caml_alloc_string(self.len());
            let ptr = crate::sys::string_val(value);
            core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

unsafe impl FromValue for &[u8] {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe { ::core::slice::from_raw_parts(ptr, len) }
    }
}

unsafe impl ToValue for &[u8] {
    fn to_value(self) -> Value {
        unsafe {
            let value = crate::sys::caml_alloc_string(self.len());
            let ptr = crate::sys::string_val(value);
            core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

unsafe impl FromValue for &mut [u8] {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }
    }
}

unsafe impl ToValue for &mut [u8] {
    fn to_value(self) -> Value {
        unsafe {
            let value = crate::sys::caml_alloc_string(self.len());
            let ptr = crate::sys::string_val(value);
            core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: ToValue> ToValue for Vec<V> {
    fn to_value(self) -> Value {
        let len = self.len();
        let mut arr = Value::alloc(len, Tag(0));

        for (i, v) in self.into_iter().enumerate() {
            arr.store_field(i, v.to_value());
        }

        arr
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<V: FromValue> FromValue for Vec<V> {
    fn from_value(v: Value) -> Vec<V> {
        unsafe {
            let len = crate::sys::caml_array_length(v.0);
            let mut dst = Vec::with_capacity(len);
            for i in 0..len {
                dst.push(V::from_value(Value(*crate::sys::field(v.0, i))))
            }
            dst
        }
    }
}

unsafe fn as_slice<'a>(value: Value) -> &'a [Value] {
    ::core::slice::from_raw_parts(
        (value.0 as *const Value).offset(-1),
        crate::sys::wosize_val(value.0) + 1,
    )
}

unsafe impl<'a> FromValue for &'a [Value] {
    fn from_value(v: Value) -> &'a [Value] {
        unsafe { as_slice(v) }
    }
}

unsafe impl<'a> FromValue for &'a mut [Value] {
    fn from_value(v: Value) -> &'a mut [Value] {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                (v.0 as *mut Value).offset(-1),
                crate::sys::wosize_val(v.0) + 1,
            )
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: Ord + FromValue, V: FromValue> FromValue for std::collections::BTreeMap<K, V> {
    fn from_value(v: Value) -> std::collections::BTreeMap<K, V> {
        let mut dest = std::collections::BTreeMap::new();

        let mut tmp = v;
        while tmp.0 != crate::sys::EMPTY_LIST {
            let (k, v) = tmp.field(0);
            dest.insert(k, v);
            tmp = tmp.field(1);
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: ToValue, V: ToValue> ToValue for std::collections::BTreeMap<K, V> {
    fn to_value(self) -> Value {
        let mut list = crate::List::empty();

        self.into_iter().rev().for_each(|(k, v)| {
            let k = k.to_value();
            let v = v.to_value();
            list = list.add((k, v));
        });

        list.to_value()
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: FromValue> FromValue for std::collections::LinkedList<T> {
    fn from_value(v: Value) -> std::collections::LinkedList<T> {
        let mut dest = std::collections::LinkedList::new();

        let mut tmp = v;
        while tmp.0 != crate::sys::EMPTY_LIST {
            let t = tmp.field(0);
            dest.push_back(t);
            tmp = tmp.field(1);
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: ToValue> ToValue for std::collections::LinkedList<T> {
    fn to_value(self) -> Value {
        let mut list = crate::List::empty();

        self.into_iter().rev().for_each(|t| {
            let t = t.to_value();
            list = list.add(t);
        });
        list.to_value()
    }
}

unsafe impl ToValue for &Value {
    fn to_value(self) -> Value {
        self.clone().to_value()
    }
}
