use crate as ocaml;

use crate::{Error, FromValue, ToValue, Value};

#[test]
fn test_basic_array() -> Result<(), Error> {
    ocaml::runtime::init();
    let mut a: ocaml::Array<&str> = ocaml::Array::alloc(2);
    a.set(0, "testing")?;
    a.set(1, "123")?;
    let b: Vec<&str> = FromValue::from_value(a.to_value());
    assert!(b.as_slice() == &["testing", "123"]);
    Ok(())
}

#[ocaml::func]
pub fn make_tuple(a: Value, b: Value) -> (Value, Value) {
    (a, b)
}

#[test]
fn test_tuple_of_tuples() {
    ocaml::runtime::init();

    ocaml::body!(() {
        let x = (1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64).to_value();
        let y = (9f64, 8f64, 7f64, 6f64, 5f64, 4f64, 3f64, 2f64, 1f64).to_value();
        let ((a, b, c, d, e, f, g, h, i), (j, k, l, m, n, o, p, q, r)): (
            (f64, f64, f64, f64, f64, f64, f64, f64, f64),
            (f64, f64, f64, f64, f64, f64, f64, f64, f64),
        ) = FromValue::from_value(make_tuple(x, y));

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

#[test]
fn test_basic_list() {
    ocaml::runtime::init();
    ocaml::body!(() {
        let mut list = ocaml::List::empty();
        let a = 3i64.to_value();
        let b = 2i64.to_value();
        let c = 1i64.to_value();
        list = list.add(a);
        list = list.add(b);
        list = list.add(c);

        assert!(list.len() == 3);

        let ll: std::collections::LinkedList<i64> = FromValue::from_value(list.to_value());

        for (i, x) in ll.into_iter().enumerate() {
            assert!((i + 1) as i64 == x);
        }
    })
}
