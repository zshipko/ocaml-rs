use ocaml::{FromValue, ToValue};

#[derive(ToValue, FromValue)]
enum Enum11 {
    Empty,
    First(ocaml::Int),
    Second(ocaml::Array<&'static str>),
}

#[ocaml::func]
pub fn enum1_empty() -> Enum11 {
    Enum11::Empty
}

#[ocaml::func]
pub fn enum1_first(i: ocaml::Int) -> Enum11 {
    Enum11::First(i)
}

#[ocaml::func]
pub fn enum1_make_second(s: &'static str) -> Enum11 {
    let mut arr = ocaml::Array::alloc(1);
    let _ = arr.set(0, s);
    Enum11::Second(arr)
}

#[ocaml::func]
pub fn enum1_get_second_value(e: Enum11) -> Option<ocaml::Array<&'static str>> {
    match e {
        Enum11::Second(x) => Some(x),
        Enum11::Empty | Enum11::First(_) => None,
    }
}

#[ocaml::func]
pub fn enum1_is_empty(e: Enum11) -> bool {
    match e {
        Enum11::Empty => true,
        _ => false,
    }
}
