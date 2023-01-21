## 1.0.0-beta.5

- Implement `ToValue` for `Box<T>` by @fmckeogh https://github.com/zshipko/ocaml-rs/pull/107
- Refactor derive macros for `FromValue` and `ToValue`
- Drop support for OCaml 4.07

## 1.0.0-beta.4

- Added `ocaml::function` macro for calling OCaml values
- Fix spelling in book by @fmckeogh in https://github.com/zshipko/ocaml-rs/pull/98
- Fixes no_std by @fmckeogh in https://github.com/zshipko/ocaml-rs/pull/99
- Feature gate panic hook by @fmckeogh in https://github.com/zshipko/ocaml-rs/pull/100
- Test `no_std` feature in CI by @fmckeogh in https://github.com/zshipko/ocaml-rs/pull/101
- Implement `FromValue` for `Box<T>` by @fmckeogh in https://github.com/zshipko/ocaml-rs/pull/105

## 1.0.0-beta.2

- Added `Seq` type

## 1.0.0-beta.1

- Add `ToValue`/`FromValue` implementations for `u32`

## 1.0.0-beta.0

- Removed `IntoValue` and added `ToValue` because it now accepts a reference to self
- `Custom` types now have to be wrapped in a `Pointer<T>`
- Added `ocaml::import!` macro for calling OCaml functions from Rust
- Added `ocaml::sig` proc-macro for generating `external` and type signatures
- Added ocaml-build crate for generating OCaml code from `ocaml::sig` macros and linking dune
  projects
- Renamed `Value::call` to `Value::call1` and rewrote `Value::call` to take a variable number of
  arguments
- Added support for automatic conversion between OCaml `Result.t` and Rust `Result`
- Renamed `Value::float` to `Value::double` and `Value::float_val` to `Value::double_val`
- Added `Value::alloc_double_array`, `Value::double_field` and `Value::store_double_field`
- Improved support for float arrays in ocaml-sys
- `Custom` values have a new default `finalize` implementation that will drop the inner Rust value

## 0.22.4

- Added `Value::exn_to_string` to convert OCaml exception values to their string representation
- Added `gc_minor`, `gc_major`, `gc_full_major` and `gc_compact` functions for interacting with
  the OCaml garbage collector

## 0.22.3

- Use latest `ocaml-interop`

## 0.22.2

- Adds `FromValue`/`ToValue` for `[u8]`

## 0.22.1

- Add `no-caml-startup` feature to allow `ocaml-rs` libraries to link
  correctly when using `dune utop`

## 0.22.0

- Allow `Value` to hold boxroot or raw value
- Add `Raw::as_value` and `Raw::as_pointer`

## 0.21.0

- New `Value` implementation to use `ocaml-boxroot-sys`
  * `Value` no longer implements `Copy`
- `ocaml::Raw` was added to wrap `ocaml::sys::Value` in macros
- Update `ocaml-interop` version

## 0.20.1

- Fix issue with OCaml runtime initialization: https://github.com/zshipko/ocaml-rs/pull/59

## 0.20.0

- `Value` methods marked as `unsafe`: the `Value` API is considered the "unsafe" API and `ocaml-interop` is the safer choice
- `ToValue` renamed to `IntoValue`
- All functions that cause OCaml allocations (including `IntoValue::into_value`) take a reference to `ocaml::Runtime`, which is provided by
  an implicit variable named `gc` when using `ocaml-derive` (the name of this variable is configurable: `#[ocaml::func(my_gc_var)]`)
