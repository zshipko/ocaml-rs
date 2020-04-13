use crate as ocaml;

use crate::{Error, FromValue, ToValue, Value};

#[ocaml::func]
pub fn make_tuple(a: Value, b: Value) -> (Value, Value) {
    (a, b)
}

macro_rules! setup {
    ($x:block) => {{
        ocaml::runtime::init();
        ocaml::runtime::acquire_lock();
        let f = || $x;
        let x = f();
        ocaml::runtime::release_lock();
        x
    }};
}

#[test]
fn basic_array() -> Result<(), Error> {
    setup!({
        let mut a: ocaml::Array<&str> = ocaml::Array::alloc(2);
        a.set(0, "testing")?;
        a.set(1, "123")?;
        let b: Vec<&str> = FromValue::from_value(a.to_value());
        assert!(b.as_slice() == &["testing", "123"]);
        Ok(())
    })
}

#[test]
fn make_tuple_of_tuples() {
    setup!({
        let ((a, b, c, d, e, f, g, h, i), (j, k, l, m, n, o, p, q, r)): (
            (f64, f64, f64, f64, f64, f64, f64, f64, f64),
            (f64, f64, f64, f64, f64, f64, f64, f64, f64),
        ) = FromValue::from_value(make_tuple(
            (1., 2., 3., 4., 5., 6., 7., 8., 9.).to_value(),
            (9., 8., 7., 6., 5., 4., 3., 2., 1.).to_value(),
        ));

        println!("a: {}, r: {}", a, r);
        assert!(a == r);
        assert!(b == q);
        assert!(c == p);
        assert!(d == o);
        assert!(e == n);
        assert!(f == m);
        assert!(g == l);
        assert!(h == k);
        assert!(i == j);
    })
}
