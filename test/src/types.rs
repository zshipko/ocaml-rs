use ocaml::interop::ToOCaml;
use ocaml::Value;

#[ocaml::func]
#[ocaml::sig("'a list -> int")]
pub unsafe fn list_length(x: ocaml::List<ocaml::Value>) -> ocaml::OCaml<ocaml::interop::OCamlInt> {
    ocaml::OCaml::of_i32(x.len() as i32)
}

#[ocaml::func]
#[ocaml::sig("unit -> 'a list")]
pub fn list_nil() -> ocaml::List<ocaml::Value> {
    ocaml::List::empty()
}

#[ocaml::func]
#[ocaml::sig("'a list -> 'a -> 'a list")]
pub unsafe fn list_cons(
    l: ocaml::List<ocaml::Value>,
    x: ocaml::Value,
) -> ocaml::List<ocaml::Value> {
    l.add(gc, &x)
}

#[ocaml::func]
#[ocaml::sig("int -> int -> int array")]
pub unsafe fn array_make_range(
    start: ocaml::Uint,
    stop: ocaml::Uint,
) -> Result<ocaml::Array<ocaml::Value>, ocaml::Error> {
    let len = stop - start;
    let mut arr = ocaml::Array::alloc(len);

    for i in 0..len {
        arr.set(gc, i, &Value::uint(i + start))?;
    }
    Ok(arr)
}

#[ocaml::func]
#[ocaml::sig("int -> int -> float array")]
pub fn array_make_range_f(start: isize, stop: isize) -> Vec<f64> {
    (start..stop).map(|x| x as f64).collect()
}

#[ocaml::func]
#[ocaml::sig("'a array -> int -> 'a -> 'a option")]
pub unsafe fn array_replace(
    mut arr: ocaml::Array<ocaml::Value>,
    index: ocaml::Uint,
    x: Value,
) -> Result<Option<Value>, ocaml::Error> {
    let y = arr.get(gc, index)?;
    arr.set(gc, index, &x)?;
    Ok(Some(y))
}

#[ocaml::func]
#[ocaml::sig("string -> (int, int8_unsigned_elt, c_layout) Array1.t")]
pub unsafe fn array1_of_string(x: String) -> ocaml::bigarray::Array1<u8> {
    ocaml::bigarray::Array1::from_slice(x.as_bytes())
}

#[ocaml::func]
#[ocaml::sig("int -> init:int -> (int, int8_unsigned_elt, c_layout) Array1.t")]
pub unsafe fn array1_new(len: ocaml::Uint, init: u8) -> ocaml::bigarray::Array1<u8> {
    let mut ba = ocaml::bigarray::Array1::<u8>::create(len as usize);
    let data = ba.data_mut();
    for i in data {
        *i = init;
    }
    ba
}

#[ocaml::func]
#[ocaml::sig("unit -> (float, float32_elt, c_layout) Array1.t")]
pub unsafe fn array1_from_rust_vec() -> ocaml::bigarray::Array1<f32> {
    ocaml::bigarray::Array1::from_slice(&[1f32, 2f32, 3f32, 4f32, 5f32])
}

#[ocaml::func]
pub unsafe fn make_array2(dim1: usize, dim2: usize) -> ocaml::bigarray::Array2<f32> {
    let arr = ndarray::Array2::zeros((dim1, dim2));
    ocaml::bigarray::Array2::from_ndarray(arr)
}

#[ocaml::func]
pub fn array2_set(mut arr: ocaml::bigarray::Array2<f32>, x: usize, y: usize, v: f32) {
    let mut view = arr.view_mut();
    view[[x, y]] = v;
}

#[ocaml::func]
pub fn array2_get(
    arr: ocaml::bigarray::Array2<f32>,
    x: usize,
    y: usize,
) -> ocaml::OCaml<ocaml::interop::OCamlFloat> {
    let view = arr.view();
    let item = view[[x, y]] as f64;
    item.to_ocaml(gc)
}

#[ocaml::func]
pub fn array2_format(arr: ocaml::bigarray::Array2<f32>) -> String {
    format!("{}", arr.view()).replace('\n', "")
}

#[derive(Debug)]
struct Abstract {
    f: f64,
}

#[ocaml::func]
pub unsafe fn alloc_abstract_pointer(f: ocaml::Float) -> Value {
    let a = Box::into_raw(Box::new(Abstract { f }));
    Value::alloc_abstract_ptr(a)
}

#[ocaml::func]
pub unsafe fn abstract_pointer_value(f: Value) -> ocaml::OCaml<ocaml::interop::OCamlFloat> {
    let f = f.abstract_ptr_val::<Abstract>();
    let x = (*f).f;
    x.to_ocaml(gc)
}

#[ocaml::func]
pub unsafe fn abstract_pointer_free(f: Value) {
    let f = f.abstract_ptr_val_mut::<Abstract>();
    drop(Box::from_raw(f));
}
