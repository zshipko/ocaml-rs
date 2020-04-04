#[macro_use]
extern crate ocaml;
use ocaml::core::state;
use ocaml::{ToValue, Value};

caml!(ml_send_int(v){
    caml_local!(l);
    let x = v.int_val();
    l = 0xbeef.to_value();
    println!("send_int  0x{:x}", x);
    return l;
});

caml!(ml_send_two, |v, v2|, <a>, {
    println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &state::local_roots(), state::local_roots(), v.value());
    let tag: u8 = v2.tag().into();
    println!("string tag: {}", tag);
    let x = v.int_val();
    let string = ocaml::Str::from(v2);
    println!("got  0x{:x}, {}", x, string.as_str());
});

caml!(ml_send_tuple(t) {
    caml_local!(dest);
    let x = t.field(0).int_val();
    let y = t.field(1).int_val();

    dest = (x + y).to_value();
    return dest;
});

caml!(ml_send_int64(x) {
    let _x = x.int64_val();
    return Value::int64(_x + 10i64);
});

caml!(ml_new_tuple(i) {
    let i = i.int_val();
    return tuple!(i, i * 2, i * 3).into();
});

caml!(ml_new_array(i) {
    let i = i.int_val();
    let x: Vec<isize> = (0..5).map(|x| x * i).collect();
    return x.to_value();
});

caml!(ml_new_list(i){
    let i = i.int_val();
    return list!(0 * i, 1 * i, 2 * i, 3 * i, 4 * i).into();
});

caml!(ml_testing_callback(a, b) {
    let f = ocaml::named_value("print_testing")
        .expect("print_testing not registered");

    f.call_n(&[a, b]).unwrap();
    return Value::unit();
});

caml!(ml_raise_not_found(_unit){
    ocaml::raise_not_found();
    return Value::unit();
});

caml!(ml_send_float(f){
    return (f.f64_val() * 2.0).to_value();
});

caml!(ml_send_first_variant(_unit) {
    return Value::variant(0, Some(2.0))
});

extern "C" fn finalizer(_value: ocaml::core::Value) {
    println!("Finalizer");
}

caml!(ml_custom_value(_unit) {
    return Value::alloc_custom(1, finalizer);
});

caml!(ml_array1(len) {
    let mut ba = ocaml::Array1::<u8>::create(len.int_val() as usize);
    for i in 0..ba.len() {
        ba.data_mut()[i] = i as u8;
    }
    return ba.into();
});

caml!(ml_array2(s) {
    let mut a: ocaml::Str = s.into();
    let b = a.data_mut();
    let ba = ocaml::Array1::<u8>::of_slice(b); // Note: `b` is still owned by OCaml since it was passed as a parameter
    return ba.into();
});

caml!(ml_string_test(s){
    let st = ocaml::Str::from(s.clone());
    println!("{:?}", s.tag());
    println!("{} {}", st.len(), st.as_str());
    return ocaml::Str::from("testing").into();
});

caml!(ml_make_list(length) {
    let length = length.int_val();
    let mut list = ocaml::List::new();
    let mut sum_list = 0;
    for v in 0..length {
        sum_list += v;
        list.push_hd(Value::int(v));
    }

    // list to vec
    let vec: Vec<Value> = list.to_vec();
    println!("vec.len: {:?}", vec.len());
    assert_eq!(list.len(), vec.len());
    let mut sum_vec = 0;
    for i in 0..vec.len() {
        let v = vec[i].int_val();
        sum_vec += v;
    }

    // check heads
    let list_hd = list.hd().unwrap().int_val();
    let vec_hd = vec[0].int_val();
    println!("list_hd: {:?} vs. vec_hd: {:?}", list_hd, vec_hd);
    assert_eq!(list_hd, vec_hd);

    // check sums
    println!("sum_list: {:?} vs. sum_vec: {:?}", sum_list, sum_vec);
    assert_ne!(0, sum_list);
    assert_ne!(0, sum_vec);
    assert_eq!(sum_list, sum_vec);

    return list.into();
});

caml!(ml_make_array(length) {
    let length = length.int_val() as usize;
    let mut arr = ocaml::Array::new(length);
    for v in 0..length {
        arr.set(v, Value::int(v as isize)).unwrap();
    }
    arr.into()
});

caml!(ml_call(f, a) {
    f.call_exn(a).unwrap()
});
