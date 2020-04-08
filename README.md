# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was originally forked from [raml](https://crates.io/crates/raml), but has been almost entirely re-written thanks to support from the [OCaml Software Foundation](http://ocaml-sf.org/).

Works with OCaml versions `4.06.0` and up

Please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

### Getting started

**OCaml**:

Take a look at [example/src/dune](https://github.com/zshipko/ocaml-rs/blob/master/example/src/dune) for an example `dune` file to get you started.

**Rust**

Typically just include:

```toml
ocaml = {git = "https://github.com/zshipko/ocaml-rs"}
```

in your `Cargo.toml`.



On macOS you will need also to add the following to your project's `.cargo/config` file:

```toml
[build]
rustflags = ["-C", "link-args=-Wl,-undefined,dynamic_lookup"]
```

This is because macOS doesn't allow undefined symbols in dynamic libraries by default.

### Documentation

[https://docs.rs/ocaml](https://docs.rs/ocaml)

### Examples

```rust
// Automatically derive `ToValue` and `FromValue`
#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Example<'a> {
    name: &'a str,
    i: ocaml::Int,
}


#[ocaml::func]
pub fn incr_example(mut e: Example) -> Example {
    e.i += 1;
    e
}

#[ocaml::func]
pub fn build_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
    (i + 1, i + 2, i + 3)
}

#[ocaml::func]
pub fn average(arr: ocaml::Array<f64>) -> Result<f64, ocaml::Error> {
    let mut sum = 0f64;

    for i in 0..arr.len() {
        sum += arr.get_double(i)?;
    }

    Ok(sum / arr.len() as f64)
}

// A `bare_func` must take `ocaml::Value` for every argument and return an `ocaml::Value`
// these functions have minimal overhead compared to wrapping with `func`
#[ocaml::bare_func]
pub fn incr(value: ocaml::Value) -> ocaml::Value {
    let i = value.int_val();
    Value::int(i + 1)
}

// This is equivalent to:
#[no_mangle]
pub extern "C" fn incr2(value: ocaml::Value) -> ocaml::Value {
    ocaml::body((value) {
        let i = value.int_val();
        ocaml::Value::int( i + 1)
    })
}

// `ocaml::bare_func` ensures that #[no_mangle] and extern "C" are added, in addition to wrapping
// the function body using `ocaml::body!`
```

The OCaml stubs would look like this:

```ocaml
type example = {
    name: string;
    i: int;
}

external incr_example: example -> example = "incr_example"
external build_tuple: int -> int * int * int = "build_tuple"
external average: float array -> float = "average"
external incr: int -> int = "incr"
```

For more examples see [./example](https://github.com/zshipko/ocaml-rs/blob/master/example) or [ocaml-vec](https://github.com/zshipko/ocaml-vec) for an example project using `ocaml-rs`.

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

