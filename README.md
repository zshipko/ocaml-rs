# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

**Note:** `ocaml-rs` is still experimental, please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was forked from [raml](https://crates.io/crates/raml) with the goal of creating a safer, high-level interface.

### Documentation

[https://docs.rs/ocaml](https://docs.rs/ocaml)

### Examples:

```rust
use ocaml::*;

caml!(build_tuple(i) {
    let i = i.val_i32();
    Tuple::from(&[i + 1, i + 2, i + 3])
});

caml!(average(arr) {
    let arr = Array::from(arr);
    let len = arr.len();
    let sum = 0f64;

    for i in 0..len {
        sum += arr.get_double_unchecked(i);
    }

    Value::f64(sum / len as f64)
})
```

This will take care of all the OCaml garbage collector related bookkeeping (CAMLparam, CAMLlocal and CAMLreturn).

In OCaml the stubs for these functions looks like this:

```ocaml
external build_tuple: int -> int * int * int = "build_tuple"
external average: float array -> float = "average"
```

For more examples see [./example](https://github.com/zshipko/ocaml-rs/blob/master/example) or [ocaml-vec](https://github.com/zshipko/ocaml-vec).

### `caml!` macro

The old style `caml!` macro has been replaced with a much simpler new format.

Instead of:

```rust
caml!(function_name, |a, b, c|, <local> {
    ...
} -> local);
```

you can now write:

```rust
caml!(function_name(a, b, c) {
    caml_local!(local);
    ...
    return local;
});
```

However, when using the type wrappers provided by `ocaml-rs` (`Array`, `List`, `Tuple`, `Str`, `Array1`, ...), `caml_local!` is already called internally. This means that the following is valid without having to declare a local value for the result:

```rust
caml!(example(a, b, c){
    List::from(&[a, b, c])
});
```
