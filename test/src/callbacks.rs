use ocaml::{Error, Value};

#[ocaml::func]
pub fn apply1(f: Value, x: Value) -> Result<Value, Error> {
    f.call(x)
}

#[ocaml::func]
pub fn apply3(f: Value, x: Value) -> Result<Value, Error> {
    f.call(f.call(f.call(x)))
}

#[ocaml::func]
pub fn apply_range(f: Value, start: ocaml::Int, stop: ocaml::Int) -> Result<Value, Error> {
    let mut l = ocaml::List::empty();
    for i in start..stop {
        l = l.add(stop - 1 - i)
    }

    f.call(l)
}
