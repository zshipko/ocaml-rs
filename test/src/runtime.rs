use ocaml::Value;

#[no_mangle]
pub extern "C" fn unboxed_float_avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[ocaml::bytecode_func]
pub fn ml_unboxed_float_bytecode(a: f64, b: f64) -> f64 {
    unboxed_float_avg(a, b)
}

#[ocaml::func]
pub unsafe fn more_than_five_params(
    a: ocaml::Float,
    b: ocaml::Float,
    c: ocaml::Float,
    d: ocaml::Float,
    e: ocaml::Float,
    f: ocaml::Float,
    g: ocaml::Float,
) -> ocaml::Float {
    a + b + c + d + e + f + g
}

#[ocaml::func]
pub fn raise_exc(x: ocaml::Float) -> Result<(), ocaml::Error> {
    ocaml::Error::raise_with_arg("Exc", x)
}

#[ocaml::func]
pub fn hash_variant_abc(i: ocaml::Int) -> Value {
    Value::hash_variant("Abc", Some(Value::int(i)))
}

#[ocaml::func]
pub fn hash_variant_def(i: ocaml::Float) -> Value {
    Value::hash_variant("Def", Some(Value::float(i)))
}
