#[macro_use]
extern crate ocaml;

// use std::io;
// use std::io::Write;

caml!(ml_send_int, |v, v2|, <l>, {
    let x = int_val!(v);
    l = val_int!(0xbeef);
    println!("send_int  0x{:x}", x);
    // io::stdout().flush();
} -> l);

caml!(ml_send_two, |v, v2|, {
    println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &ocaml::memory::caml_local_roots, ocaml::memory::caml_local_roots, v);
    let x = int_val!(v);
    let len = ocaml::mlvalues::caml_string_length(v2);
    let ptr = string_val!(v2);
    let slice = ::std::slice::from_raw_parts(ptr, len);
    let string = ::std::str::from_utf8_unchecked(slice);
    println!("got  0x{:x}, {}", x, string);
});

caml!(ml_send_tuple, |t|, <dest>, {
    let x = int_val!(*field!(t, 0));
    let y = int_val!(*field!(t, 1));

    dest = val_int!(x + y);
} -> dest);

caml!(ml_new_tuple, |unit|, <dest, a, b, c>, {
    a = val_int!(0);
    b = val_int!(1);
    c = val_int!(2);

    let mut tuple = ocaml::Tuple::new(3);
    tuple.set(0, a).unwrap();
    tuple.set(1, b).unwrap();
    tuple.set(2, c).unwrap();
    dest = ocaml::Value::from(tuple);
} -> dest);
