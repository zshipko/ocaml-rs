#[macro_use]
extern crate ocaml;
use ocaml::core::memory;
use ocaml::{ToValue, Value};

caml!(ml_send_int, |v|, <l>, {
    let x = v.i32_val();
    l = 0xbeef.to_value();
    println!("send_int  0x{:x}", x);
} -> l);

caml!(ml_send_two, |v, v2|, {
    println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &memory::caml_local_roots, memory::caml_local_roots, v.value());
    let tag: u8 = v2.tag().into();
    println!("string tag: {}", tag);
    let x = v.i32_val();
    let string = ocaml::Str::from(v2);
    println!("got  0x{:x}, {}", x, string.as_str());
});

caml!(ml_send_tuple, |t|, <dest>, {
    let x = t.field(0).i32_val();
    let y = t.field(1).i32_val();

    dest = (x + y).to_value()
} -> dest);

caml!(ml_send_int64, |x|, <dest>, {
    let _x = x.int64_val();

    dest = Value::int64(_x + 10i64)
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

caml!(ml_raise_not_found, |_unit|, {
    ocaml::raise_not_found()
});

caml!(ml_send_float, |f|, <dest>, {
    dest = (f.f64_val() * 2.0).to_value()
} -> dest);

caml!(ml_send_first_variant, |_unit|, <dest>, {
    dest = Value::variant(0, Some(2.0))
} -> dest);

extern "C" fn finalizer(_value: ocaml::core::Value) {
    println!("Finalizer");
}

caml!(ml_custom_value, |_unit|, <dest>, {
    let t = Value::alloc_custom(1, finalizer);
    dest = t.into()
} -> dest);

caml!(ml_array1, |_unit|, <dest>, {
    let mut ba = ocaml::Array1::<u8>::create(100);
    for i in 0..ba.len() {
        ba.data_mut()[i] = i as u8;
    }
    dest = ba.into();
} -> dest);

caml!(ml_string_test, |s|, <dest>, {
    let st = ocaml::Str::from(s.clone());
    println!("{:?}", s.tag());
    println!("{} {}", st.len(), st.as_str());
    dest = ocaml::Str::from("testing").into();
} -> dest);
