#[macro_use]
extern crate ocaml;
use ocaml::Value;
use ocaml::core::memory;

caml!(ml_send_int, |v|, <l>, {
    let x = v.int_val();
    l = Value::int(0xbeef);
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

    dest = Value::int(x + y)
} -> dest);

caml!(ml_new_tuple, |_unit|, <dest>, {
    let tuple = ocaml::Tuple::from(vec![Value::int(0), Value::int(1), Value::int(2)]);
    dest = Value::from(tuple);
} -> dest);

caml!(ml_new_array, |_unit|, <dest>, {
    let x: Vec<Value> = (0..5).map(|x| Value::int(x)).collect();
    dest = ocaml::Array::from(x).into();
} -> dest);

caml!(ml_new_list, |_unit|, <dest>, {
    let x: Vec<Value> = (0..5).map(|x| Value::int(x)).collect();
    dest = ocaml::List::from(x).into();
} -> dest);

caml!(ml_testing_callback, |a, b|, {
    let f = ocaml::named_value("print_testing")
        .expect("print_testing not registered");

    f.call_n(vec![a, b]).unwrap();
});
