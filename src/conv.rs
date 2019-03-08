use crate::types::{Array, Str};
use crate::value::{FromValue, ToValue, Value};

macro_rules! value_i {
    ($t:ty) => {
        impl ToValue for $t {
            fn to_value(&self) -> $crate::Value {
                $crate::Value::i64(self.clone() as i64)
            }
        }

        impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                v.i64_val() as $t
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

value_i!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
value_f!(f32, f64);

impl ToValue for String {
    fn to_value(&self) -> Value {
        let s = Str::from(self.as_str());
        Value::from(s)
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> String {
        let s = Str::from(v);
        String::from(s.as_str())
    }
}

impl<V: ToValue> ToValue for Vec<V> {
    fn to_value(&self) -> Value {
        let tmp: Vec<Value> = self.iter().map(|x| x.to_value()).collect();
        crate::array!(_ tmp).into()
    }
}

impl<V: FromValue> FromValue for Vec<V> {
    fn from_value(mut v: Value) -> Vec<V> {
        let arr = Array::from(&mut v);
        let mut dst = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            dst.push(V::from_value(arr.get(i).unwrap()))
        }
        dst
    }
}
