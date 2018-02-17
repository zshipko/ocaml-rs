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
    let string = ocaml::Str::from(v2);
    println!("got  0x{:x}, {}", x, string.as_str());
});

caml!(ml_send_tuple, |t|, <dest>, {
    let x = t.field(0).int_val();
    let y = t.field(1).int_val();

    dest = Value::int(x + y)
} -> dest);

caml!(ml_new_tuple, |_unit|, <dest>, {
    let tuple = ocaml::Tuple::from((Value::int(0), Value::int(1), Value::int(2)));
    dest = Value::from(tuple);
} -> dest);

caml!(ml_new_array, |_unit|, <dest>, {
    let mut arr = ocaml::Array::new(5);

    for i in 0..5 {
        arr.set(i, Value::int(i as i32)).unwrap();
    }

    dest = Value::from(arr)
} -> dest);

caml!(ml_new_list, |_unit|, <dest>, {
    let mut list = ocaml::List::new();

    for i in 0..5 {
        list.append(Value::int(i));
    }

    dest = ocaml::Value::from(list)
} -> dest);
