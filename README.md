# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was forked from [raml](https://crates.io/crates/raml) with the goal of creating a safer, high-level interface.

```rust
#[macro_use]
use ocaml;
use ocaml::ToValue;

caml!(ml_beef, |parameter|, <local>, {
    let i = parameter.val_int();
    let res = 0xbeef * i;
    println!("about to return  0x{:x} to OCaml runtime", res);
    local = res.to_value();
} -> local);
```

This will take care of all the OCaml garbage collector related bookkeeping (CAMLparam, CAMLlocal and CAMLreturn)

For a working example see `./examples/rust` and `./examples/ocaml`

### Documentation

https://docs.rs/ocaml/

