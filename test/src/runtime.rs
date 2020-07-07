use ocaml::Value;

#[no_mangle]
pub extern "C" fn unboxed_float_avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[ocaml::bytecode_func]
pub fn unboxed_float_avg_bytecode(a: f64, b: f64) -> f64 {
    unboxed_float_avg(a, b)
}

#[ocaml::func]
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
pub fn raise_exc(x: ocaml::Float) -> Result<(), ocaml::Error> {
    ocaml::Error::raise_with_arg("Exc", x)
}

#[ocaml::func]
pub fn raise_failure() -> Result<(), ocaml::Error> {
    ocaml::Error::failwith("An error")
}

#[ocaml::func]
pub fn hash_variant_abc(i: ocaml::Int) -> Value {
    Value::hash_variant("Abc", Some(Value::int(i)))
}

#[ocaml::func]
pub fn hash_variant_def(i: ocaml::Float) -> Value {
    Value::hash_variant("Def", Some(Value::float(i)))
}

#[ocaml::func]
pub fn test_panic() -> ocaml::Int {
    panic!("XXX")
}

#[ocaml::func]
pub fn test_call_named(g: ocaml::Float) -> Result<ocaml::Value, ocaml::Error> {
    let f: Value = Value::named("call_named").unwrap();
    f.call(g)
}
