extern crate ocaml;

#[test]
fn test_memory() {
    let caml_int = 7471857119 as usize; // if this is not cast, then it's a u32 and conversion will fail
    let x = ocaml::Value::new(caml_int).int_val();
    assert_eq!(x, 0xdeadbeef);
}
