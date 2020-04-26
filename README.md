# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was originally forked from [raml](https://crates.io/crates/raml), but has been almost entirely re-written thanks to support from the [OCaml Software Foundation](http://ocaml-sf.org/).

Works with OCaml versions `4.06.0` and up

Please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

### Getting started

**OCaml**:

Take a look at [test/src/dune](https://github.com/zshipko/ocaml-rs/blob/master/test/src/dune) for an example `dune` file to get you started.

**Rust**

Typically just include:

```toml
ocaml = "0.11"
```

or

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

Additionally, if you plan on releasing to OPAM, you will need to vendor your Rust dependencies to avoid making network requests during the build phase, since reaching out to crates.io/github will be blocked by the OPAM sandbox.

### Features

- `derive`
  * enabled by default, adds `#[ocaml::func]` and friends and `derive` implementations for `FromValue` and `ToValue`
- `link`
  * link the native OCaml runtime, this enables `ocaml::runtime::init`, which is equivalent to `caml_main`
- `no-std`
  * Allows `ocaml` to be used in `#![no_std]` environments like MirageOS
- `deep-clone`
  * enables `Value::deep_clone_to_ocaml` and `Value::deep_clone_to_rust`

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

// A `native_func` must take `ocaml::Value` for every argument and return an `ocaml::Value`
// these functions have minimal overhead compared to wrapping with `func`
#[ocaml::native_func]
pub fn incr(value: ocaml::Value) -> ocaml::Value {
    let i = value.int_val();
    ocaml::Value::int(i + 1)
}

// This is equivalent to:
#[no_mangle]
pub extern "C" fn incr2(value: ocaml::Value) -> ocaml::Value {
    ocaml::body!((value) {
        let i = value.int_val();
        ocaml::Value::int( i + 1)
    })
}

// `ocaml::native_func` is responsible for:
// - Ensures that #[no_mangle] and extern "C" are added, in addition to wrapping
// - Wraps the function body using `ocaml::body!`

// Finally, if your function is marked [@@unboxed] and [@@noalloc] in OCaml then you can avoid
// boxing altogether for f64 arguments using a plain C function and a bytecode function
// definition:
#[no_mangle]
pub extern "C" fn incrf(input: f64) -> f64 {
    input + 1.0
}

#[cfg(feature = "derive")]
#[ocaml::bytecode_func]
pub fn incrf_bytecode(input: f64) -> f64 {
    incrf(input)
}
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
external incr2: int -> int = "incr2"
external incrf: float -> float = "incrf_bytecode" "incrf" [@@unboxed] [@@noalloc]
```

For more examples see [./test](https://github.com/zshipko/ocaml-rs/blob/master/test) or [ocaml-vec](https://github.com/zshipko/ocaml-vec) for an example project using `ocaml-rs`.

### Type conversion

This chart contains the mapping between Rust and OCaml types used by `ocaml::func`

| Rust type        | OCaml type           |
| ---------------- | -------------------- |
| `()`             | `unit`               |
| `isize`          | `int`                |
| `usize`          | `int`                |
| `i8`             | `int`                |
| `u8`             | `int`                |
| `i16`            | `int`                |
| `u16`            | `int`                |
| `i32`            | `int32`              |
| `u32`            | `int32`              |
| `i64`            | `int64`              |
| `u64`            | `int64`              |
| `f32`            | `float`              |
| `f64`            | `float`              |
| `str`            | `string`             |
| `String`         | `string`             |
| `Option<A>`      | `'a option`          |
| `Result<A, B>`   | `exception`          |
| `(A, B, C)`      | `'a * 'b * 'c`       |
| `&[Value]`       | `'a array` (no copy) |
| `Vec<A>`, `&[A]` | `'a array`           |
| `BTreeMap<A, B>` | `('a, 'b) list`      |
| `LinkedList<A>`  | `'a list`            |

Even though `&[Value]` is specifically marked as no copy, a type like `Option<Value>` would also qualify since the inner value is not converted to a Rust type. However, `Option<String>` will do full unmarshaling into Rust types. Another thing to note: `FromValue` for `str` and `&[u8]` is zero-copy, however `ToValue` for `str` and `&[u8]` creates a new value - this is necessary to ensure the string is registered with the OCaml runtime.

If you're concerned with minimizing allocations/conversions you should use `Value` type directly.

## Upgrading

Since 0.10 and later have a much different API compared to earlier version, here is are some major differences that should be considered when upgrading:

- `FromValue` and `ToValue` have been marked `unsafe` because converting OCaml values to Rust and back also depends on the OCaml type signature.
  * A possible solution to this would be a `cbindgen` like tool that generates the correct OCaml types from the Rust code
- `ToValue` now takes ownership of the value being converted
- The `caml!` macro has been rewritten as a procedural macro called `ocaml::func`, which performs automatic type conversion
  * `ocaml::native_func` and `ocaml::bytecode_func` were also added to create functions at a slightly lower level
  * `derive` feature required
- Added `derive` implementations for `ToValue` and `FromValue` for stucts and enums
  * `derive` feature required
- `i32` and `u32` now map to OCaml's `int32` type rather than the `int` type
  * Use `ocaml::Int`/`ocaml::Uint` to refer to the OCaml's `int` types now
- `Array` and `List` now take generic types
- Strings are converted to `str` or `String`, rather than using the `Str` type
- Tuples are converted to Rust tuples (up to 20 items), rather than using the `Tuple` type
- The `core` module has been renamed to `sys` and is now just an alias for the `ocaml-sys` crate and all sub-module have been removed
