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
