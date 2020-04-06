# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was forked from [raml](https://crates.io/crates/raml), but has been almost entirely re-written thanks to support from the [OCaml Software Foundation](http://ocaml-sf.org/).

Works with OCaml versions `4.06.0` and up

Please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

### Documentation

[https://docs.rs/ocaml](https://docs.rs/ocaml)

### Examples:

```rust
// Automatically derive `ToValue` and `FromValue`
#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Example<'a> {
    name: &'a str,
    i: ocaml::Int,
}

#[ocaml::func]
pub fn struct_example(e: Example) -> ocaml::Int {
    e.i + 1
}

#[ocaml::func]
pub fn build_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
    (i + 1, i + 2, i + 3)
}

#[ocaml::func]
pub fn average(arr: ocaml::Value) -> Result<f64, ocaml::Error> {
    let len = ocaml::array::len(arr);
    let mut sum = 0f64;

    for i in 0..len {
        sum += ocaml::array::get_double(arr, i)?;
    }

    Ok(sum / len as f64)
}
```

This will take care of all the OCaml garbage collector related bookkeeping (CAMLparam, CAMLlocal and CAMLreturn).

The OCaml stubs would look like this:

```ocaml
type example = {
    name: string;
    i: int;
}
//!
external struct_example: example -> int = "struct_example"
external build_tuple: int -> int * int * int = "build_tuple"
external average: float array -> float = "average"
```

For more examples see [./example](https://github.com/zshipko/ocaml-rs/blob/master/example) or [ocaml-vec](https://github.com/zshipko/ocaml-vec).

### Type conversion

This chart contains the mapping between Rust and OCaml types used by `ocaml::func`

| Rust type        | OCaml type      |
| ---------------- | --------------- |
| `()`             | `unit`          |
| `isize`          | `int`           |
| `usize`          | `int`           |
| `i8`             | `int`           |
| `u8`             | `int`           |
| `i16`            | `int`           |
| `u16`            | `int`           |
| `i32`            | `int32`         |
| `u32`            | `int32`         |
| `i64`            | `int64`         |
| `u64`            | `int64`         |
| `f32`            | `float`         |
| `f64`            | `float`         |
| `str`            | `string`        |
| `String`         | `string`        |
| `Option<'a>`     | `'a option`     |
| `(A, B, C)`      | `'a * 'b * 'c`  |
| `Vec<A>`         | `'a array`      |
| `BTreeMap<A, B>` | `('a, 'b) list` |
| `LinkedList<A>`  | `'a list`       |

