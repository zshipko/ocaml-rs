#[macro_use]
extern crate ocaml;
use ocaml::sys::state;
use ocaml::{FromValue, ToValue, Value};

use std::collections::LinkedList;

#[ocaml::func]
pub fn ml_send_int(x: isize) -> isize {
    println!("send_int  0x{:x}", x);
    0xbeef
}

#[ocaml::bare_func]
pub fn ml_send_two(v: Value, v2: Value) -> Value {
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
    Value::unit()
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
pub fn ml_new_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
    (i, i * 2, i * 3)
}

#[ocaml::func]
pub fn ml_new_array(i: ocaml::Int) -> Vec<ocaml::Int> {
    (0..5).map(|x| x * i).collect()
}

#[ocaml::func]
pub fn ml_new_list(i: ocaml::Int) -> LinkedList<ocaml::Int> {
    let mut l = LinkedList::new();
    l.push_back(0 * i);
    l.push_back(1 * i);
    l.push_back(2 * i);
    l.push_back(3 * i);
    l.push_back(4 * i);
    l
}

#[ocaml::func]
pub fn ml_testing_callback(a: Value, b: Value) {
    let f = ocaml::named_value("print_testing").expect("print_testing not registered");

    f.call_n(&[a, b]).unwrap();
}

#[ocaml::func]
pub fn ml_raise_not_found() {
    ocaml::raise_not_found()
}

#[ocaml::func]
pub fn ml_send_float(f: f64) -> f64 {
    f * 2.0
}

#[derive(ToValue, FromValue)]
enum Testing {
    First(f64),
    Second(ocaml::Int),
}

#[ocaml::func]
pub fn ml_send_first_variant() -> Testing {
    Testing::First(2.0)
}

extern "C" fn finalizer(value: Value) {
    let ptr: ocaml::Opaque<&str> = ocaml::Opaque::from_value(value);
    println!("Finalizer: {}", ptr.data());
}

#[ocaml::func]
pub fn ml_custom_value() -> ocaml::Opaque<'static, &'static str> {
    let x = ocaml::Opaque::new(&mut "testing", Some(finalizer));
    assert!(x.data() == &"testing");
    x
}

#[ocaml::func]
pub fn ml_array1(len: ocaml::Int) -> ocaml::bigarray::Array1<'static, u8> {
    let mut ba = ocaml::bigarray::Array1::<u8>::create(len as usize);
    for i in 0..ba.len() {
        ba.data_mut()[i] = i as u8;
    }
    return ba;
}

#[ocaml::func]
pub fn ml_array2(s: &mut str) -> ocaml::bigarray::Array1<u8> {
    let ba = unsafe {
        ocaml::bigarray::Array1::of_slice(s.as_bytes_mut()) // Note: `b` is still owned by OCaml since it was passed as a parameter
    };
    return ba;
}

#[ocaml::func]
pub fn ml_string_test(s: Value) -> &'static str {
    let st: &str = FromValue::from_value(s);
    println!("{:?}", s.tag());
    println!("{} {}", st.len(), st);
    "testing"
}

#[ocaml::func]
pub fn ml_make_list(length: ocaml::Int) -> ocaml::List<'static, ocaml::Int> {
    let mut sum_list = 0;
    let mut list = ocaml::List::empty();
    for v in 0..length {
        sum_list += v;
        list.push_hd(length - v - 1);
    }

    // list to vec
    let vec: Vec<ocaml::Int> = list.to_vec();
    println!("vec.len: {:?}", vec.len());
    assert_eq!(list.len(), vec.len());
    let mut sum_vec = 0;
    for i in 0..vec.len() {
        let v = vec[i];
        sum_vec += v;
    }

    // check heads
    let list_hd: ocaml::Int = list.hd().unwrap();
    let vec_hd = vec[0];
    println!("list_hd: {:?} vs. vec_hd: {:?}", list_hd, vec_hd);
    assert_eq!(list_hd, vec_hd);

    // check sums
    println!("sum_list: {:?} vs. sum_vec: {:?}", sum_list, sum_vec);
    assert_ne!(0, sum_list);
    assert_ne!(0, sum_vec);
    assert_eq!(sum_list, sum_vec);

    list
}

#[ocaml::func]
pub fn ml_make_array(
    length: ocaml::Int,
) -> Result<ocaml::Array<'static, ocaml::Int>, ocaml::Error> {
    let mut value = ocaml::Array::alloc(length as usize);
    for v in 0..length {
        value.set(v as usize, v)?;
    }
    Ok(value)
}

#[ocaml::func]
pub fn ml_call(f: Value, a: Value) -> Result<Value, ocaml::Error> {
    f.call(a)
}

#[derive(ToValue, FromValue, Debug)]
struct MyRecord<'a> {
    foo: &'a str,
    bar: f64,
}

#[ocaml::func]
pub fn ml_format_my_record(s: MyRecord) -> String {
    format!("{:?}", s)
}
