# Writing OCaml functions in Rust

This section requires the `derive` feature, which is enabled in `ocaml-rs` by default. This exposes `ocaml::func`, which is the recommended way to create an OCaml function in Rust. Below are some examples using `ocaml::func`

- [Hello world](#hello-world)
- [Structs and enums](#structs-and-enums)
- [Calling an OCaml function](#calling-an-ocaml-function)
- [Abstract types](#abstract-types)
- [Raising an exception](#raising-an-exception)
- [Returning OCaml result](#returning-ocaml-resut)
- [Using `Value` directly](#using-value-directly)
- [Unboxed arguments](#unboxed-arguments)

## Hello world

This example returns a string from Rust to OCaml

```rust
# extern crate ocaml;
#[ocaml::func]
#[ocaml::sig("unit -> string")]
pub fn hello_world() -> &'static str {
  "Hello, world!"
}
```

## Structs and enums

The example uses `derive(ToValue)` and `derive(FromValue)` to create an enum and struct that can be used as parameters to `ocaml::func`s

```rust
# extern crate ocaml;

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("Add | Sub | Mul | Div")]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div
}

#[derive(ocaml::FromValue, ocaml::ToValue)]
#[ocaml::sig("lhs: float; rhs: float; op: bin_op")]
pub struct Expr {
  lhs: f64,
  rhs: f64,
  op: BinOp,
}

#[ocaml::func]
#[ocaml::sig("expr -> float")]
pub fn expr_eval(expr: Expr) -> f64 {
  match expr.op {
    BinOp::Add => expr.lhs + expr.rhs,
    BinOp::Sub => expr.lhs - expr.rhs,
    BinOp::Mul => expr.lhs * expr.rhs,
    BinOp::Div => expr.lhs / expr.rhs
  }
}

```

## Calling an OCaml function

This example shows how to call an OCaml function from Rust - the OCaml function must be registered using `Callback.register`. In this case we're calling the OCaml function `my_incr`, which looks like this:

```ocaml
let my_incr x = x + 1
let () = Callback.register "my_incr" my_incr
```

```rust
# extern crate ocaml;

ocaml::import! {
  fn my_incr(x: ocaml::Int) -> ocaml::Int;
}

#[ocaml::func]
#[ocaml::sig("int -> int")]
pub unsafe fn call_my_incr(x: ocaml::Int) -> Result<ocaml::Int, ocaml::Error> {
  my_incr(gc, x)
}
```

A few things to note:

- When calling the `import!`ed function you will need to pass the OCaml runtime handle as the first parameter
- The return value of the function will be wrapped in `Result<T, ocaml::Error>` because the function may raise an exception

## Abstract types

This example shows how to wrap a Rust type using `ocaml::Pointer`

```rust
# extern crate ocaml;

use std::io::Read;

#[ocaml::sig("")] // Creates an abstract type on the OCaml side
type File = std::fs::File;

#[ocaml::func]
#[ocaml::sig("string -> file")]
pub fn file_open(filename: &str) -> Result<ocaml::Pointer<File>, ocaml::Error> {
  let f = File::open(filename)?;
  Ok(ocaml::Pointer::alloc(f))
}

#[ocaml::func]
#[ocaml::sig("file -> string")]
pub fn file_read(mut file :ocaml::Pointer<File>) -> Result<String, ocaml::Error> {
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
```

## Raising an exception

Raising an exception is accomplished by panicking:


```rust
# extern crate ocaml;

#[ocaml::func]
#[ocaml::sig("int -> unit")]
pub unsafe fn fail_if_even_panic(i: ocaml::Int) {
  if i % 2 == 0 {
    panic!("even")
  }
}
```

or returning a `Result<_, ocaml::Error>` value:


```rust
# extern crate ocaml;

#[ocaml::func]
#[ocaml::sig("int -> unit")]
pub unsafe fn fail_if_even_result(i: ocaml::Int) -> Result<(), ocaml::Error> {
  if i % 2 == 0 {
    return Err(ocaml::Error::Caml(ocaml::CamlError::Failure("even")))
  }

  Ok(())
}
```

## Returning OCaml result

In the previous example `Result<_, ocaml::Error>` was used to raise an exception, however `Result<A, B>` where `A` and `B` both implement `ToValue` will create an OCaml `('a, 'b) Result.t`:

```rust
# extern crate ocaml;
use ocaml::{ToValue};

#[ocaml::func]
#[ocaml::sig("string -> (int, [`Msg of string]) result")]
pub unsafe fn try_int_of_string(s: &str) -> Result<ocaml::Int, ocaml::Value> {
  match s.parse::<isize>() {
    Ok(i) => Ok(i),
    Err(e) => {
      let s = format!("{e:?}");
      let err = ocaml::Value::hash_variant(gc, "Msg", Some(s.to_value(gc)));
      Err(err)
    }
  }
}
```

## Using `Value` directly

It is also possible to use `ocaml::Value` to avoid any conversion or copying, however this can be more error prone.

```rust
# extern crate ocaml;

#[ocaml::func]
#[ocaml::sig("string array -> int -> string -> unit")]
pub unsafe fn array_set(mut array: ocaml::Value, index: ocaml::Value, s: ocaml::Value) {
  array.store_field(gc, index.int_val() as usize, s)
}
```

## Unboxed arguments

Unfortunately `ocaml::func` doesn't support unboxed/noalloc functions, however it is still possible to create them using `ocaml-rs`:

```rust
# extern crate ocaml;

#[no_mangle]
pub extern "C" fn unboxed_float_avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[ocaml::bytecode_func]
pub fn unboxed_float_avg_bytecode(a: f64, b: f64) -> f64 {
    unboxed_float_avg(a, b)
}
```

In this case you will also need to write the signature manually:

```ocaml
external unboxed_float_avg: float -> float -> float = "unboxed_float_avg_bytecode" "unboxed_float_avg" [@@unboxed] [@@noalloc]
```
