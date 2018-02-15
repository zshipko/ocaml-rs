# ocaml-rs - OCaml FFI for Rust

NOTE: `ocaml-rs` is based on `raml v0.1`

Direct OCaml bindings without ever leaving Rust - no C stubs!

(you still have to know how the C ffi bindings work; if you do, the macros are almost identical to the C ones in their naming and purpose)

Please see the example in `examples` for the Rust code in `rust` for the Rust code that OCaml code will call and the `ocaml` directory for the OCaml code that calls the Rust code.

Also, please bear with me as I'm trying to add more documentation and examples, but I am very busy; if you see something, don't hesitate to add a PR or issue, thanks :)

A basic example demonstrates their usage:

```rust
caml!(ml_beef, |parameter|, <local>, {
    let i = int_val!(parameter);
    let res = 0xbeef * i;
    println!("about to return  0x{:x} to OCaml runtime", res);
    local = val_int!(res);
} -> local);
```

The macro takes care of _automatically_ declaring `CAMLparam` et. al, as well as `CAMLlocal` and `CAMLreturn`.

If you need more fine grained control, `caml_body!` and others are available.

### Documentation

https://docs.rs/ocaml/

