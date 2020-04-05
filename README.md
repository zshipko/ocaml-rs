# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

**Note:** `ocaml-rs` is still experimental, please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was forked from [raml](https://crates.io/crates/raml) with the goal of creating a safer, high-level interface.

Works with OCaml versions `4.06.0` and up

### Documentation

[https://docs.rs/ocaml](https://docs.rs/ocaml)

### Examples:

```rust
#[ocaml::func]
pub fn build_tuple(i: isize) -> (isize, isize, isize) {
    (i + 1, i + 2, i + 3)
};

#[ocaml::func]
pub fn average(arr: ocaml::Value) -> Result<f64, ocaml::Error> {
    let len = ocaml::array::len(arr);
    let mut sum = 0f64;

    for i in 0..len {
        sum += ocaml::array::get_double(arr, i)?;
    }

    Ok(sum / len as f64)
};
```

This will take care of all the OCaml garbage collector related bookkeeping (CAMLparam, CAMLlocal and CAMLreturn).

In OCaml the stubs for these functions looks like this:

```ocaml
external build_tuple: int -> int * int * int = "build_tuple"
external average: float array -> float = "average"
```

For more examples see [./example](https://github.com/zshipko/ocaml-rs/blob/master/example) or [ocaml-vec](https://github.com/zshipko/ocaml-vec).
