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

// i32, u32, i64, u64,
value_i!(i8, u8, i16, u16, isize, usize);
value_f!(f32, f64);

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

impl ToValue for str {
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
        let mut arr = crate::alloc(len, 0);

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
