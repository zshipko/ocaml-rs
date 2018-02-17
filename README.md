# ocaml-rs - OCaml FFI for Rust

NOTE: `ocmal-rs` is based on [raml v0.1](https://crates.io/crates/raml) with the goal of creating an even higher-level interface.

Direct OCaml bindings without ever leaving Rust - no C stubs!

Please see the example in `examples` for the Rust code in `rust` for the Rust code that OCaml code will call and the `ocaml` directory for the OCaml code that calls the Rust code.

```rust
caml!(ml_beef, |parameter|, <local>, {
    let i = parameter.val_int();
    let res = 0xbeef * i;
    println!("about to return  0x{:x} to OCaml runtime", res);
    local = Value::int(res);
} -> local);
```

The macro takes care of _automatically_ declaring `CAMLparam` et. al, as well as `CAMLlocal` and `CAMLreturn`.

### Documentation

https://docs.rs/ocaml/

