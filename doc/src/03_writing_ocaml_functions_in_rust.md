# Writing OCaml functions in Rust

This section requires the `derive` feature, which is enabled in `ocaml-rs` by default. This exposes `ocaml::func`, which is the recommended way to create an OCaml function in Rust.

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

## `derive(ToValue)` and `derive(FromValue)`

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
#[ocaml::sig("{lhs: float; rhs: float; op: bin_op}")]
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


