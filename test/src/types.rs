use ocaml::Value;

#[ocaml::func]
pub fn list_length(x: ocaml::List<ocaml::Value>) -> ocaml::Int {
    x.len() as ocaml::Int
}

#[ocaml::func]
pub fn list_nil() -> ocaml::List<ocaml::Value> {
    ocaml::List::empty()
}

#[ocaml::func]
pub fn list_cons(l: ocaml::List<ocaml::Value>, x: ocaml::Value) -> ocaml::List<ocaml::Value> {
    l.add(gc, x)
}

#[ocaml::func]
pub unsafe fn array_make_range(
    start: ocaml::Uint,
    stop: ocaml::Uint,
) -> Result<ocaml::Array<ocaml::Value>, ocaml::Error> {
    let len = stop - start;
    let mut arr = ocaml::Array::alloc(gc, len);

    for i in 0..len {
        arr.set(gc, i, Value::uint(i + start))?;
    }
    Ok(arr)
}

#[ocaml::func]
pub fn array_make_range_f(start: isize, stop: isize) -> Vec<f64> {
    (start..stop).map(|x| x as f64).collect()
}

#[ocaml::func]
pub unsafe fn array_replace(
    mut arr: ocaml::Array<ocaml::Value>,
    index: ocaml::Uint,
    x: Value,
) -> Result<Option<Value>, ocaml::Error> {
    let y = arr.get(index)?;
    arr.set(gc, index, x)?;
    Ok(Some(y))
}

#[ocaml::func]
pub unsafe fn array1_of_string(x: &mut str) -> ocaml::bigarray::Array1<u8> {
    ocaml::bigarray::Array1::of_slice(gc, x.as_bytes_mut())
}

#[ocaml::func]
pub fn array1_new(len: ocaml::Uint, init: u8) -> ocaml::bigarray::Array1<u8> {
    let mut ba = ocaml::bigarray::Array1::<u8>::create(gc, len as usize);
    let data = ba.data_mut();
    for i in data {
        *i = init;
    }
    ba
}

#[ocaml::func]
pub fn array1_from_rust_vec() -> ocaml::bigarray::Array1<f32> {
    ocaml::bigarray::Array1::from_slice(gc, &[1f32, 2f32, 3f32, 4f32, 5f32])
}

#[ocaml::func]
pub fn make_array2(dim1: usize, dim2: usize) -> ocaml::bigarray::Array2<f32> {
    let arr = ndarray::Array2::zeros((dim1, dim2));
    ocaml::bigarray::Array2::from_ndarray(gc, arr)
}

#[ocaml::func]
pub fn array2_set(mut arr: ocaml::bigarray::Array2<f32>, x: usize, y: usize, v: f32) {
    let mut view = arr.view_mut();
    view[[x, y]] = v;
}

#[ocaml::func]
pub fn array2_get(arr: ocaml::bigarray::Array2<f32>, x: usize, y: usize) -> f32 {
    let view = arr.view();
    view[[x, y]]
}

#[ocaml::func]
pub fn array2_format(arr: ocaml::bigarray::Array2<f32>) -> String {
    format!("{}", arr.view()).replace("\n", "")
}

#[derive(Debug)]
struct Abstract {
    f: f64,
}

#[ocaml::func]
pub unsafe fn alloc_abstract_pointer(f: ocaml::Float) -> Value {
    let a = Box::into_raw(Box::new(Abstract { f }));
    Value::alloc_abstract_ptr(gc, a)
}

#[ocaml::func]
pub unsafe fn abstract_pointer_value(f: Value) -> ocaml::Float {
    let f = f.abstract_ptr_val::<Abstract>();
    (*f).f
}

#[ocaml::func]
pub unsafe fn abstract_pointer_free(f: Value) {
    let f = f.abstract_ptr_val_mut::<Abstract>();
    Box::from_raw(f);
}
