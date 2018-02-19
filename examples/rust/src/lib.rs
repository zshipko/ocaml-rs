#[macro_use]
extern crate ocaml;
use ocaml::{ToValue, Value};
use ocaml::core::memory;

caml!(ml_send_int, |v|, <l>, {
    let x = v.int_val();
    l = 0xbeef.to_value();
    println!("send_int  0x{:x}", x);
} -> l);

caml!(ml_send_two, |v, v2|, {
    println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &memory::caml_local_roots, memory::caml_local_roots, v.value());
    let x = v.int_val();
    println!("string tag: {}", v2.tag() as u8);
    let string = ocaml::Str::from(v2);
    println!("got  0x{:x}, {}", x, string.as_str());
});

caml!(ml_send_tuple, |t|, <dest>, {
    let x = t.field(0).int_val();
    let y = t.field(1).int_val();

    dest = (x + y).to_value()
} -> dest);

caml!(ml_new_tuple, |_unit|, <dest>, {
    dest = tuple!(0i32, 1i32, 2i32);
} -> dest);

caml!(ml_new_array, |_unit|, <dest>, {
    let x: Vec<i32> = (0..5).collect();
    dest = x.to_value();
} -> dest);

caml!(ml_new_list, |_unit|, <dest>, {
    dest = list!(0i32, 1i32, 2i32, 3i32, 4i32);
} -> dest);

caml!(ml_testing_callback, |a, b|, {
    let f = ocaml::named_value("print_testing")
        .expect("print_testing not registered");

    f.call_n(&[a, b]).unwrap();
});
