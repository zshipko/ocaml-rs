use ocaml::{interop::ToOCaml, FromValue, IntoValue};

#[derive(IntoValue, FromValue)]
#[ocaml::sig("Empty | First of int | Second of string array")]
enum Enum1<'a> {
    Empty,
    First(ocaml::Int),
    Second(ocaml::Array<'a, String>),
}

#[ocaml::func]
#[ocaml::sig("unit -> enum1")]
pub fn enum1_empty() -> Enum1 {
    Enum1::Empty
}

#[ocaml::func]
#[ocaml::sig("int -> enum1")]
pub unsafe fn enum1_first(i: ocaml::Value) -> Enum1 {
    let i: ocaml::interop::OCaml<ocaml::Int> = i.to_ocaml(gc);
    Enum1::First(i.to_i64() as ocaml::Int)
}

#[ocaml::func(test)]
#[ocaml::sig("string -> enum1")]
pub unsafe fn enum1_make_second(s: String) -> Enum1 {
    let mut arr = ocaml::Array::alloc(1);
    let _ = arr.set(test, 0, s);
    Enum1::Second(arr)
}

#[ocaml::func]
#[ocaml::sig("enum1 -> string array option")]
pub fn enum1_get_second_value(e: Enum1<'static>) -> Option<ocaml::Array<'static, String>> {
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

#[derive(IntoValue, FromValue, Default)]
#[ocaml::sig("{a: int; b: float; mutable c: string option; d: string array option;}")]
struct Struct1 {
    a: ocaml::Int,
    b: ocaml::Float,
    c: Option<String>,
    d: Option<ocaml::Array<'static, String>>,
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
    d: Option<ocaml::Array<'static, String>>,
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
    vec![("foo", 1), ("bar", 2isize)].into_value(gc)
}

#[ocaml::native_func]
#[ocaml::sig("unit -> string array")]
pub fn string_array() -> ocaml::Value {
    let mut v = vec![];
    for i in 1..10000000 {
        v.push(format!("foo {}", i));
    }
    v.into_value(gc)
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
