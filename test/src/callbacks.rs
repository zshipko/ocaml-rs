use ocaml::{Error, Value};

#[ocaml::func]
#[ocaml::sig("('a -> 'b) -> 'a -> 'b")]
pub unsafe fn apply1(f: Value, x: Value) -> Result<Value, Error> {
    f.call(gc, x)
}

#[ocaml::func]
#[ocaml::sig("('a -> 'b) -> 'a -> 'b")]
pub unsafe fn apply3(f: Value, x: Value) -> Result<Value, Error> {
    let a = f.call(gc, x)?;
    let b = f.call(gc, a)?;
    f.call(gc, b)
}

#[ocaml::func]
#[ocaml::sig("(int list -> 'a) -> int -> int -> 'a")]
pub unsafe fn apply_range(f: Value, start: ocaml::Int, stop: ocaml::Int) -> Result<Value, Error> {
    let mut l = ocaml::List::empty();
    for i in start..stop {
        l = l.add(gc, stop - 1 - i)
    }

    f.call(gc, l)
}
