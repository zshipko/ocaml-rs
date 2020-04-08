use crate::value::{FromValue, ToValue, Value};

macro_rules! value_i {
    ($t:ty) => {
        impl ToValue for $t {
            fn to_value(&self) -> $crate::Value {
                $crate::Value::int(self.clone() as isize)
            }
        }

        impl FromValue for $t {
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
        impl ToValue for $t {
            fn to_value(&self) -> $crate::Value {
                $crate::Value::f64(self.clone().into())
            }
        }

        impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                v.f64_val() as $t
            }
        }
    };
    ($($t:ty),*) => {
        $(value_f!($t);)*
    }
}

value_i!(i8, u8, i16, u16, crate::Int, crate::Uint);
value_f!(f32, f64);

impl ToValue for i64 {
    fn to_value(&self) -> crate::Value {
        Value::int64(*self)
    }
}

impl FromValue for i64 {
    fn from_value(v: crate::Value) -> i64 {
        v.int64_val()
    }
}

impl ToValue for u64 {
    fn to_value(&self) -> crate::Value {
        Value::int64(*self as i64)
    }
}

impl FromValue for u64 {
    fn from_value(v: crate::Value) -> u64 {
        v.int64_val() as u64
    }
}

impl ToValue for i32 {
    fn to_value(&self) -> crate::Value {
        Value::int32(*self)
    }
}

impl FromValue for i32 {
    fn from_value(v: crate::Value) -> i32 {
        v.int32_val()
    }
}

impl ToValue for u32 {
    fn to_value(&self) -> crate::Value {
        Value::int64(*self as i64)
    }
}

impl FromValue for u32 {
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
        impl<$($t: FromValue),*> FromValue for ($($t,)*) {
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

        impl<$($t: ToValue),*> ToValue for ($($t,)*) {
            fn to_value(&self) -> crate::Value {
                #[allow(unused)]
                let mut len = 0;
                $(
                    #[allow(unused)]
                    {
                        len = $n;
                    }
                )*

                let mut v = $crate::Value::alloc(len, 0);
                $(
                    v.store_field($n, $t::to_value(&self.$n));
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

impl ToValue for bool {
    fn to_value(&self) -> Value {
        Value::int(*self as isize)
    }
}

impl FromValue for bool {
    fn from_value(v: Value) -> bool {
        v.int_val() != 0
    }
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        unsafe {
            let value = crate::sys::alloc::caml_alloc_string(self.len());
            let ptr = crate::sys::mlvalues::string_val(value);
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

impl FromValue for String {
    fn from_value(value: Value) -> String {
        let len = unsafe { crate::sys::mlvalues::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::mlvalues::string_val(value.0) };
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, len);
            ::std::str::from_utf8(slice).expect("Invalid UTF-8").into()
        }
    }
}

impl ToValue for () {
    fn to_value(&self) -> Value {
        Value::unit()
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        if value == Value::none() {
            return None;
        }

        Some(value.field(0))
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        match self {
            Some(x) => Value::some(x.to_value()),
            None => Value::none(),
        }
    }
}

impl FromValue for &str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::mlvalues::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::mlvalues::string_val(value.0) };
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, len);
            ::std::str::from_utf8(slice).expect("Invalid UTF-8")
        }
    }
}

impl ToValue for &str {
    fn to_value(&self) -> Value {
        unsafe {
            let value = crate::sys::alloc::caml_alloc_string(self.len());
            let ptr = crate::sys::mlvalues::string_val(value);
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

impl FromValue for &mut str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::mlvalues::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::mlvalues::string_val(value.0) };
        unsafe {
            let slice = ::std::slice::from_raw_parts_mut(ptr, len);
            ::std::str::from_utf8_mut(slice).expect("Invalid UTF-8")
        }
    }
}

impl ToValue for &mut str {
    fn to_value(&self) -> Value {
        unsafe {
            let value = crate::sys::alloc::caml_alloc_string(self.len());
            let ptr = crate::sys::mlvalues::string_val(value);
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Value(value)
        }
    }
}

impl<V: ToValue> ToValue for Vec<V> {
    fn to_value(&self) -> Value {
        let tmp: Vec<Value> = self.iter().map(|x| x.to_value()).collect();
        let len = tmp.len();
        let mut arr = Value::alloc(len, 0);

        for (i, v) in tmp.into_iter().enumerate() {
            arr.store_field(i, v);
        }

        arr
    }
}

impl<V: FromValue> FromValue for Vec<V> {
    fn from_value(v: Value) -> Vec<V> {
        unsafe {
            let len = crate::sys::mlvalues::caml_array_length(v.0);
            let mut dst = Vec::with_capacity(len);
            for i in 0..len {
                dst.push(V::from_value(Value(*crate::sys::mlvalues::field(v.0, i))))
            }
            dst
        }
    }
}

impl<V: ToValue> ToValue for &[V] {
    fn to_value(&self) -> Value {
        let len = self.len();
        let mut arr = Value::alloc(len, 0);
        for (i, v) in self.iter().enumerate() {
            arr.store_field(i, v.to_value());
        }

        arr
    }
}

unsafe fn as_slice<'a>(value: Value) -> &'a [Value] {
    ::std::slice::from_raw_parts(
        (value.0 as *const Value).offset(-1),
        crate::sys::mlvalues::wosize_val(value.0) + 1,
    )
}

impl<'a> FromValue for &'a [Value] {
    fn from_value(v: Value) -> &'a [Value] {
        unsafe { as_slice(v) }
    }
}

impl<T: ToValue, E: std::fmt::Debug> ToValue for Result<T, E> {
    fn to_value(&self) -> Value {
        match self {
            Ok(x) => x.to_value(),
            Err(e) => {
                let s = format!("{:?}", e);
                crate::failwith(s);
                Value::unit()
            }
        }
    }
}

impl<T: FromValue, E> FromValue for Result<T, E> {
    fn from_value(value: Value) -> Result<T, E> {
        Ok(T::from_value(value))
    }
}

impl<K: Ord + FromValue, V: FromValue> FromValue for std::collections::BTreeMap<K, V> {
    fn from_value(v: Value) -> std::collections::BTreeMap<K, V> {
        let mut dest = std::collections::BTreeMap::new();

        let mut tmp = v;
        while tmp.0 != crate::sys::mlvalues::EMPTY_LIST {
            let (k, v) = tmp.field(0);
            dest.insert(k, v);
            tmp = tmp.field(1);
        }

        dest
    }
}

impl<K: ToValue, V: ToValue> ToValue for std::collections::BTreeMap<K, V> {
    fn to_value(&self) -> Value {
        let mut list = crate::List::empty();

        self.iter().rev().for_each(|(k, v)| {
            let k = k.to_value();
            let v = v.to_value();
            list.push_hd((k, v));
        });

        list.to_value()
    }
}

impl<T: FromValue> FromValue for std::collections::LinkedList<T> {
    fn from_value(v: Value) -> std::collections::LinkedList<T> {
        let mut dest = std::collections::LinkedList::new();

        let mut tmp = v;
        while tmp.0 != crate::sys::mlvalues::EMPTY_LIST {
            let t = tmp.field(0);
            dest.push_front(t);
            tmp = tmp.field(1);
        }

        dest
    }
}

impl<T: ToValue> ToValue for std::collections::LinkedList<T> {
    fn to_value(&self) -> Value {
        let mut list = crate::List::empty();

        self.iter().rev().for_each(|t| {
            let t = t.to_value();
            list.push_hd(t);
        });
        list.to_value()
    }
}
