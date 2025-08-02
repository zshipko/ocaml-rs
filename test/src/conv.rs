use ocaml::{FromValue, ToValue};

#[derive(ToValue, FromValue)]
#[ocaml::sig("Empty | First of int | Second of string array")]
enum Enum1 {
    Empty,
    First(ocaml::Int),
    Second(ocaml::Array<String>),
}

#[ocaml::func]
#[ocaml::sig("unit -> enum1")]
pub fn enum1_empty() -> Enum1 {
    Enum1::Empty
}

#[ocaml::func]
#[ocaml::sig("int -> enum1")]
pub unsafe fn enum1_first(i: ocaml::Int) -> Enum1 {
    Enum1::First(i)
}

#[ocaml::func(test)]
#[ocaml::sig("string -> enum1")]
pub unsafe fn enum1_make_second(s: String) -> Enum1 {
    let mut arr = ocaml::Array::alloc(1);
    let _ = arr.set(test, 0, &s);
    Enum1::Second(arr)
}

#[ocaml::func]
#[ocaml::sig("enum1 -> string array option")]
pub fn enum1_get_second_value(e: Enum1) -> Option<ocaml::Array<String>> {
    match e {
        Enum1::Second(x) => Some(x),
        Enum1::Empty | Enum1::First(_) => None,
    }
}

#[ocaml::func]
#[ocaml::sig("enum1 -> bool")]
pub fn enum1_is_empty(e: Enum1) -> bool {
    matches!(e, Enum1::Empty)
}

#[derive(ToValue, FromValue, Default)]
#[ocaml::sig("{a: int; b: float; mutable c: string option; d: string array option;}")]
struct Struct1 {
    a: ocaml::Int,
    b: ocaml::Float,
    c: Option<String>,
    d: Option<ocaml::Array<String>>,
}

#[ocaml::func]
#[ocaml::sig("unit -> struct1")]
pub fn struct1_empty() -> Struct1 {
    Struct1::default()
}

#[ocaml::func]
#[ocaml::sig("struct1 -> string option")]
pub fn struct1_get_c(s: Struct1) -> Option<String> {
    s.c
}

#[ocaml::func]
#[ocaml::sig("struct1 -> string array option")]
pub fn struct1_get_d(s: Struct1) -> Option<ocaml::Array<String>> {
    s.d
}

#[ocaml::func]
#[ocaml::sig("struct1 -> string -> struct1")]
pub fn struct1_set_c(mut s: Struct1, v: String) -> Struct1 {
    s.c = Some(v);
    s
}

#[ocaml::func]
#[ocaml::sig("int -> float -> string option -> string array option -> struct1")]
#[allow(clippy::unnecessary_wraps)]
pub unsafe fn make_struct1(
    a: ocaml::Int,
    b: ocaml::Float,
    c: Option<String>,
    d: Option<ocaml::Array<String>>,
) -> Result<Struct1, ocaml::Error> {
    Ok(Struct1 { a, b, c, d })
}

#[ocaml::func]
#[ocaml::sig("string -> string")]
pub unsafe fn string_non_copying(s: ocaml::Value) -> ocaml::Value {
    s
}

#[ocaml::func]
#[ocaml::sig("int64 array -> int64")]
pub unsafe fn direct_slice(data: &[ocaml::Raw]) -> i64 {
    let mut total = 0;
    for i in data {
        total += ocaml::Value::new(*i).int64_val();
    }
    total
}

#[ocaml::func]
#[ocaml::sig("'a -> 'a")]
pub unsafe fn deep_clone(a: ocaml::Value) -> ocaml::Value {
    let b = a.deep_clone_to_rust();
    b.deep_clone_to_ocaml()
}

#[ocaml::func]
#[ocaml::sig("unit -> (string * int) array")]
pub fn pair_vec() -> ocaml::Value {
    vec![("foo", 1), ("bar", 2isize)].to_value(gc)
}

#[ocaml::native_func]
#[ocaml::sig("unit -> string array")]
pub fn string_array() -> ocaml::Value {
    let mut v = vec![];
    for i in 1..10000000 {
        v.push(format!("foo {i}"));
    }
    v.to_value(gc)
}

#[ocaml::func]
#[ocaml::sig("bytes -> bytes")]
#[allow(clippy::manual_memcpy)]
pub fn array_conv(a: [u8; 5]) -> [u8; 7] {
    let mut b = [0u8; 7];
    for i in 0..5 {
        b[i] = a[i];
        b[5] += a[i];
    }
    b[6] = 255;
    b
}

#[ocaml::func]
#[ocaml::sig("'a -> ('a, 'b) result")]
pub fn result_ok(x: ocaml::Value) -> Result<ocaml::Value, ocaml::Value> {
    Ok(x)
}

#[ocaml::func]
#[ocaml::sig("'a -> ('b, 'a) result")]
pub fn result_error(x: ocaml::Value) -> Result<ocaml::Value, ocaml::Value> {
    Err(x)
}

#[ocaml::func]
#[ocaml::sig("('a, 'b) result -> 'a option")]
pub fn result_get_ok(x: Result<ocaml::Value, ocaml::Value>) -> Option<ocaml::Value> {
    x.ok()
}

#[ocaml::func]
#[ocaml::sig("('a, 'b) result -> 'b option")]
pub fn result_get_error(x: Result<ocaml::Value, ocaml::Value>) -> Option<ocaml::Value> {
    x.err()
}

#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
#[ocaml::sig("{float_a: float; float_b: float}")]
pub struct AllFloatStruct {
    float_a: ocaml::Float,
    float_b: ocaml::Float,
}

#[ocaml::func]
#[ocaml::sig("all_float_struct -> all_float_struct")]
pub fn all_float_struct_inc_both(mut t: AllFloatStruct) -> AllFloatStruct {
    t.float_a += 1.0;
    t.float_b += 1.0;
    t
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
#[ocaml::sig("{fa: floatarray} [@@unboxed]")]
#[unboxed]
pub struct FloatArrayT {
    arr: Vec<f64>,
}

#[ocaml::func]
#[ocaml::sig("float_array_t -> floatarray")]
pub fn float_array_t_inner(f: FloatArrayT) -> Vec<f64> {
    f.arr
}

#[ocaml::func]
#[ocaml::sig("unit -> int32 array")]
pub fn make_int32_array_012() -> Vec<i32> {
    println!("make_int32_012");
    vec![0, 1, 2]
}

#[ocaml::func]
#[ocaml::sig("unit -> float array")]
pub fn make_float_array_f32_012() -> Vec<f32> {
    vec![0.0, 1.0, 2.0]
}

#[ocaml::func]
#[ocaml::sig("unit -> floatarray")]
pub fn make_floatarray_f32_012() -> Vec<f32> {
    vec![0.0, 1.0, 2.0]
}

#[ocaml::func]
#[ocaml::sig("unit -> float array")]
pub fn make_float_array_f64_012() -> Vec<f64> {
    vec![0.0, 1.0, 2.0]
}

#[ocaml::func]
#[ocaml::sig("unit -> floatarray")]
pub fn make_floatarray_f64_012() -> Vec<f64> {
    vec![0.0, 1.0, 2.0]
}
