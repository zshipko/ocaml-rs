use ocaml::{FromValue, Raw, Value};

struct Testing {
    a: ocaml::Float,
    b: i64,
    c: String,
}

unsafe extern "C" fn testing_compare(a: Raw, b: Raw) -> i32 {
    let a = Value::new(a);
    let b = Value::new(b);
    let t0 = ocaml::Pointer::<Testing>::from_value(a);
    let t1 = ocaml::Pointer::<Testing>::from_value(b);
    match (t0.as_ref().b, t1.as_ref().b) {
        (x, y) if x == y => 0,
        (x, y) if x < y => -1,
        _ => 1,
    }
}

unsafe extern "C" fn testing_finalize(a: Raw) {
    let a = Value::new(a);
    let t0 = ocaml::Pointer::<Testing>::from_value(a);
    t0.drop_in_place();
}

ocaml::custom!(Testing {
    finalize: testing_finalize,
    compare: testing_compare,
});

#[ocaml::func]
pub fn testing_alloc(b: i64) -> Testing {
    Testing {
        a: 0.0,
        b,
        c: String::new(),
    }
}

#[ocaml::func]
pub fn testing_set_c(mut testing: ocaml::Pointer<Testing>, v: String) {
    testing.as_mut().c = v;
}

#[ocaml::func]
pub fn testing_set_a(mut testing: ocaml::Pointer<Testing>, v: ocaml::Float) {
    testing.as_mut().a = v;
}

#[ocaml::func]
pub fn testing_get_values(testing: ocaml::Pointer<Testing>) -> (ocaml::Float, i64, String) {
    let t = testing.as_ref();
    (t.a, t.b, t.c.clone())
}

struct TestingCallback {
    func: ocaml::Value,
}

unsafe extern "C" fn testing_callback_finalize(a: ocaml::Raw) {
    let a = Value::new(a);
    let t0 = ocaml::Pointer::<TestingCallback>::from_value(a);
    t0.drop_in_place();
}

ocaml::custom_finalize!(TestingCallback, testing_callback_finalize);

#[ocaml::func]
pub fn testing_callback_alloc(func: ocaml::Value) -> TestingCallback {
    TestingCallback { func }
}

#[ocaml::func]
pub unsafe fn testing_callback_call(
    t: ocaml::Pointer<TestingCallback>,
    x: ocaml::Value,
) -> Result<ocaml::Value, ocaml::Error> {
    t.as_ref().func.call(gc, x)
}
