use ocaml::{IntoValue, Value};

#[no_mangle]
pub extern "C" fn unboxed_float_avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[ocaml::bytecode_func]
pub fn unboxed_float_avg_bytecode(a: f64, b: f64) -> f64 {
    unboxed_float_avg(a, b)
}

#[ocaml::func]
#[allow(clippy::too_many_arguments)]
pub unsafe fn more_than_five_params(
    mut a: ocaml::Float,
    mut b: ocaml::Float,
    c: ocaml::Float,
    d: ocaml::Float,
    e: ocaml::Float,
    f: ocaml::Float,
    g: ocaml::Float,
) -> ocaml::Float {
    a -= 1.0;
    b += 1.0;
    a + b + c + d + e + f + g
}

// See: https://github.com/zshipko/ocaml-rs/issues/29
#[ocaml::func]
pub fn mutable_parameter_with_more_than_five_arguments(
    mut net: bool,
    data: bool,
    batch_size: u64,
    epochs: u64,
    print_loss: Option<u64>,
    _metrics: Option<i32>,
) {
    let _ = net;
    let _ = data;
    let _ = batch_size;
    let _ = epochs;
    let _ = print_loss;
    net = false;
    let _ = net;
}

#[ocaml::func]
#[ocaml::sig("float -> unit")]
pub fn raise_exc(x: ocaml::Float) -> Result<(), ocaml::Error> {
    ocaml::Error::raise_with_arg("Exc", x.into_value(gc))
}

#[ocaml::func]
pub fn raise_failure() -> Result<(), ocaml::Error> {
    ocaml::Error::failwith("An error")
}

#[ocaml::func]
pub unsafe fn hash_variant_abc(i: ocaml::Int) -> Value {
    Value::hash_variant(gc, "Abc", Some(Value::int(i)))
}

#[ocaml::func]
pub unsafe fn hash_variant_def(i: ocaml::Float) -> Value {
    let f = Some(Value::f64(i));
    Value::hash_variant(gc, "Def", f)
}

#[ocaml::func]
pub fn test_panic() -> ocaml::Int {
    panic!("XXX")
}

ocaml::import! {
    fn call_named(g: ocaml::Float) -> ocaml::Float;
}

#[ocaml::func]
pub unsafe fn test_call_named(g: f64) -> Result<f64, ocaml::Error> {
    call_named(gc, &g)
}

#[ocaml::func]
pub unsafe fn bench_func() {}

#[ocaml::native_func]
pub unsafe fn bench_native_func() -> ocaml::Value {
    ocaml::Value::none()
}

#[ocaml::func]
pub unsafe fn exn_to_string(exn: ocaml::Value) -> String {
    exn.exception_to_string().unwrap()
}

#[ocaml::func]
pub unsafe fn gc_minor() {
    ocaml::runtime::gc_minor();
}

#[ocaml::func]
pub unsafe fn gc_major() {
    ocaml::runtime::gc_major();
}

#[ocaml::func]
pub unsafe fn gc_full_major() {
    ocaml::runtime::gc_full_major();
}

#[ocaml::func]
pub unsafe fn gc_compact() {
    ocaml::runtime::gc_compact();
}
