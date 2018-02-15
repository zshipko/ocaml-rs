#[macro_use]
extern crate ocaml;

#[test]
fn test_memory() {
    let caml_int = 7471857119 as usize; // if this is not cast, then it's a u32 and conversion will fail
    let x = int_val!(caml_int);
    assert_eq!(x, 0xdeadbeef);
}

