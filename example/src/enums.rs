use ocaml::{FromValue, ToValue};

#[derive(ToValue, FromValue)]
enum Enum1 {
    Empty,
    First(ocaml::Int),
    Second(ocaml::Array<&'static str>),
}

#[ocaml::func]
pub fn enum1_empty() -> Enum1 {
    Enum1::Empty
}

#[ocaml::func]
pub fn enum1_first(i: ocaml::Int) -> Enum1 {
    Enum1::First(i)
}

#[ocaml::func]
pub fn enum1_make_second(s: &'static str) -> Enum1 {
    let mut arr = ocaml::Array::alloc(1);
    let _ = arr.set(0, s);
    Enum1::Second(arr)
}

#[ocaml::func]
pub fn enum1_get_second_value(e: Enum1) -> Option<ocaml::Array<&'static str>> {
    match e {
        Enum1::Second(x) => Some(x),
        Enum1::Empty | Enum1::First(_) => None,
    }
}

#[ocaml::func]
pub fn enum1_is_empty(e: Enum1) -> bool {
    match e {
        Enum1::Empty => true,
        _ => false,
    }
}