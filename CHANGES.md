## Unreleased

- Added support for float arrays in ocaml-sys
- Renamed `Value::float` to `Value::f64` and `Value::float_val` to `Value::f64_val`
- Renamed `Array::set_double` to `Array::set_f64`, `Array::get_double` to `Array::get_f64`
  and `Array::is_double_array` to `Array::is_f64`
- Added `Value::alloc_f64_array`, `Value::f64_field` and `Value::store_f64_field`
- Added `ocaml::import!` macro for calling OCaml functions from Rust
- Added `ocaml::sig` proc-macro for generating `external` and type signatures
- Added ocaml-build crate for generating OCaml code from `ocaml::sig` macros
- Renamed `Value::call` to `Value::call1` and rewrote `Value::call` to take a variable number of
  arguments

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
