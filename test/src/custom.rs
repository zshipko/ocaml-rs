use ocaml::Raw;

#[ocaml::sig]
struct Testing {
    a: ocaml::Float,
    b: i64,
    c: String,
}

unsafe extern "C" fn testing_compare(a: Raw, b: Raw) -> i32 {
    let t0 = a.as_pointer::<Testing>();
    let t1 = b.as_pointer::<Testing>();
    match (t0.as_ref().b, t1.as_ref().b) {
        (x, y) if x == y => 0,
        (x, y) if x < y => -1,
        _ => 1,
    }
}

ocaml::custom!(Testing {
    compare: testing_compare,
});

#[ocaml::func]
#[ocaml::sig("int64 -> testing")]
pub fn testing_alloc(b: i64) -> ocaml::Pointer<Testing> {
    Testing {
        a: 0.0,
        b,
        c: String::new(),
    }
    .into()
}

#[ocaml::func]
#[ocaml::sig("testing -> string -> unit")]
pub fn testing_set_c(testing: &mut Testing, v: String) {
    testing.c = v;
}

#[ocaml::func]
#[ocaml::sig("testing -> float -> unit")]
pub fn testing_set_a(mut testing: ocaml::Pointer<Testing>, v: ocaml::Float) {
    testing.as_mut().a = v;
}

#[ocaml::func]
#[ocaml::sig("testing -> (float * int64 * string)")]
pub fn testing_get_values(t: &Testing) -> (ocaml::Float, i64, String) {
    (t.a, t.b, t.c.clone())
}

#[ocaml::sig]
struct TestingCallback {
    func: ocaml::Value,
}

unsafe extern "C" fn testing_callback_finalize(a: ocaml::Raw) {
    let t0 = a.as_pointer::<TestingCallback>();
    t0.drop_in_place();
}

ocaml::custom_finalize!(TestingCallback, testing_callback_finalize);

#[ocaml::func]
#[ocaml::sig("(int -> float) -> testing_callback")]
pub fn testing_callback_alloc(func: ocaml::Value) -> ocaml::Pointer<TestingCallback> {
    TestingCallback { func }.into()
}

#[ocaml::func]
#[ocaml::sig("testing_callback -> int -> float")]
pub unsafe fn testing_callback_call(
    t: ocaml::Pointer<TestingCallback>,
    x: ocaml::Int,
) -> Result<ocaml::Float, ocaml::Error> {
    let f = ocaml::function!(t.as_ref().func, (x: ocaml::Int) -> ocaml::Float);
    f(gc, &x)
}

// Abstract

use std::io::Read;

#[ocaml::sig]
type File = std::fs::File;

#[ocaml::func]
#[ocaml::sig("string -> file")]
pub fn file_open(filename: &str) -> Result<ocaml::Pointer<File>, ocaml::Error> {
    let f = File::open(filename)?;
    Ok(ocaml::Pointer::alloc(f))
}

#[ocaml::func]
#[ocaml::sig("file -> string")]
pub fn file_read(mut file: ocaml::Pointer<File>) -> Result<String, ocaml::Error> {
    let mut s = String::new();
    let file = file.as_mut();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[ocaml::func]
#[ocaml::sig("file -> unit")]
pub unsafe fn file_close(file: ocaml::Pointer<File>) {
    file.drop_in_place();
}
