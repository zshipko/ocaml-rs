#[macro_use]
extern crate ocaml;
use ocaml::sys::state;
use ocaml::{FromValue, ToValue, Value};

#[ocaml::func]
pub fn ml_send_int(x: isize) -> isize {
    println!("send_int  0x{:x}", x);
    0xbeef
}

#[ocaml::func]
pub fn ml_send_two(v: Value, v2: Value) {
    unsafe {
        println!(
            "local root addr: {:p} local_roots: {:#?}, v: {:?}",
            &state::local_roots(),
            state::local_roots(),
            v
        )
    };
    let tag: u8 = v2.tag().into();
    println!("string tag: {}", tag);
    let x = v.int_val();
    let string: &str = FromValue::from_value(v2);
    println!("got  0x{:x}, {}", x, string);
}

#[ocaml::func]
pub fn ml_send_tuple(t: Value) -> Value {
    local!(dest);
    let x: isize = t.field(0);
    let y: isize = t.field(1);

    dest = (x + y).to_value();
    dest
}

#[ocaml::func]
pub fn ml_send_int64(x: i64) -> i64 {
    x + 10
}

#[ocaml::func]
pub fn ml_new_tuple(i: isize) -> (isize, isize, isize) {
    (i, i * 2, i * 3)
}

caml!(fn ml_new_array(i) {
    let i = i.int_val();
    let x: Vec<isize> = (0..5).map(|x| x * i).collect();
    return x.to_value();
});

caml!(fn ml_new_list(i){
    let i = i.int_val();
    return list!(0 * i, 1 * i, 2 * i, 3 * i, 4 * i);
});

caml!(fn ml_testing_callback(a, b) {
    let f = ocaml::named_value("print_testing")
        .expect("print_testing not registered");

    f.call_n(&[a, b]).unwrap();
    return Value::unit();
});

#[ocaml::func]
pub fn ml_raise_not_found() {
    ocaml::raise_not_found();
}

caml!(fn ml_send_float(f){
    return (f.f64_val() * 2.0).to_value();
});

caml!(fn ml_send_first_variant(_unit) {
    return Value::variant(0, Some(2.0))
});

extern "C" fn finalizer(_value: Value) {
    println!("Finalizer");
}

caml!(fn ml_custom_value(_unit) {
    return ocaml::alloc_custom(1, finalizer);
});

caml!(fn ml_array1(len) {
    let mut ba = ocaml::Array1::<u8>::create(len.int_val() as usize);
    for i in 0..ba.len() {
        ba.data_mut()[i] = i as u8;
    }
    return ba;
});

caml!(fn ml_array2(s) {
    let mut a: &str = FromValue::from_value(s);
    let ba = ocaml::Array1::from(a.as_bytes()); // Note: `b` is still owned by OCaml since it was passed as a parameter
    return ba;
});

caml!(fn ml_string_test(s){
    let st: &str = FromValue::from_value(s);
    println!("{:?}", s.tag());
    println!("{} {}", st.len(), st);
    ToValue::to_value("testing")
});

caml!(fn ml_make_list(length) {
    let length = length.int_val();
    let mut list = ocaml::list::empty();
    let mut sum_list = 0;
    for v in 0..length {
        sum_list += v;
        ocaml::list::push_hd(&mut list, Value::int(v));
    }

    // list to vec
    let vec: Vec<Value> = ocaml::list::to_vec(list);
    println!("vec.len: {:?}", vec.len());
    assert_eq!(ocaml::list::len(list), vec.len());
    let mut sum_vec = 0;
    for i in 0..vec.len() {
        let v = vec[i].int_val();
        sum_vec += v;
    }

    // check heads
    let list_hd: isize = ocaml::list::hd(list).unwrap();
    let vec_hd = vec[0].int_val();
    println!("list_hd: {:?} vs. vec_hd: {:?}", list_hd, vec_hd);
    assert_eq!(list_hd, vec_hd);

    // check sums
    println!("sum_list: {:?} vs. sum_vec: {:?}", sum_list, sum_vec);
    assert_ne!(0, sum_list);
    assert_ne!(0, sum_vec);
    assert_eq!(sum_list, sum_vec);

    return list;
});

caml!(fn ml_make_array(length) {
    let length = length.int_val() as usize;
    let mut value = ocaml::alloc(length, 0);
    for v in 0..length {
        value.store_field(v, Value::int(v as isize));
    }
    value
});

caml!(fn ml_call(f, a) {
    f.call(a).unwrap()
});
