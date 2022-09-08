use ocaml::{Error, Value};

#[ocaml::func]
#[ocaml::sig("('a -> 'b) -> 'a -> 'b")]
pub unsafe fn apply1(f: Value, x: Value) -> Result<Value, Error> {
    let f = ocaml::function!(f, (a: Value) -> Value);
    f(gc, &x)
}

#[ocaml::func]
#[ocaml::sig("('a -> 'b) -> 'a -> 'b")]
pub unsafe fn apply3(f: Value, x: Value) -> Result<Value, Error> {
    let f = ocaml::function!(f, (a: Value) -> Value);
    let a = f(gc, &x)?;
    let b = f(gc, &a)?;
    f(gc, &b)
}

#[ocaml::func]
#[ocaml::sig("(int list -> 'a) -> int -> int -> 'a")]
pub unsafe fn apply_range(f: Value, start: ocaml::Int, stop: ocaml::Int) -> Result<Value, Error> {
    let mut l = ocaml::List::empty();
    for i in start..stop {
        let v = stop - 1 - i;
        l = l.add(gc, &v)
    }

    let f = ocaml::function!(f, (a: ocaml::List<ocaml::Int>) -> Value);
    f(gc, &l)
}
