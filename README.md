# ocaml-rs - OCaml extensions in Rust

<a href="https://crates.io/crates/ocaml">
    <img src="https://img.shields.io/crates/v/ocaml.svg">
</a>

`ocaml-rs` allows for OCaml extensions to be written directly in Rust with no C stubs. It was originally forked from [raml](https://crates.io/crates/raml), but has been almost entirely re-written thanks to support from the [OCaml Software Foundation](http://ocaml-sf.org/).

Works with OCaml versions `4.08.0` and up

Please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)

NOTE: While `ocaml-rs` *can* be used safely, it does not prevent a wide range of potential errors or mistakes. It should be thought of as a Rust implementation of the existing C API. [ocaml-interop](https://github.com/simplestaking/ocaml-interop) can be used to perform safe OCaml/Rust interop. `ocaml-rs` uses `ocaml-interop` behind the scenes to interact with the garbage collector. `ocaml-rs` also exports an `interop` module, which is an alias for `ocaml_interop` and the two interfaces can be combined if desired.

### Documentation

- [ocaml-rs Book](https://zshipko.github.io/ocaml-rs)
- [docs.rs](https://docs.rs/ocaml)

### Getting started

[ocaml-rust-starter](http://github.com/zshipko/ocaml-rust-starter) is a basic example to help get started with `ocaml-rs`.

On the Rust side, you will need to add the following to your `Cargo.toml`:

```toml
ocaml = "*"
```

or

```toml
ocaml = {git = "https://github.com/zshipko/ocaml-rs"}
```

For macOS you will need also to add the following to your project's `.cargo/config` file:

```toml
[build]
rustflags = ["-C", "link-args=-Wl,-undefined,dynamic_lookup"]
```

This is because macOS doesn't allow undefined symbols in dynamic libraries by default.

Additionally, if you plan on releasing to [opam](https://github.com/ocaml/opam), you will need to vendor your Rust dependencies to avoid making network requests during the build phase, since reaching out to crates.io/github will be blocked by the opam sandbox. To do this you should run:

```shell
cargo vendor
```

then follow the instructions for editing `.cargo/config`

### Build options

By default, building `ocaml-sys` will invoke the `ocamlopt` command to figure out the version and location of the OCaml compiler. There are a few environment variables to control this.

- `OCAMLOPT` (default: `ocamlopt`) is the command that will invoke `ocamlopt`
- `OCAML_VERSION` (default: result of `$OCAMLOPT -version`) is the target runtime OCaml version.
- `OCAML_WHERE_PATH` (default: result of `$OCAMLOPT -where`) is the path of the OCaml standard library.
- `OCAML_INTEROP_NO_CAML_STARTUP` (default: unset) can be set when loading an `ocaml-rs` library into an OCaml
  bytecode runtime (such as `utop`) to avoid linking issues with `caml_startup`

If both `OCAML_VERSION` and `OCAML_WHERE_PATH` are present, their values are used without invoking `ocamlopt`. If any of those two env variables is undefined, then `ocamlopt` will be invoked to obtain both values.

Defining the `OCAML_VERSION` and `OCAML_WHERE_PATH` variables is useful for saving time in CI environments where an OCaml install is not really required (to run `clippy` for example).

### Features

- `derive`
  * enabled by default, adds `#[ocaml::func]` and friends and `derive` implementations for `FromValue` and `ToValue`
- `link`
  * link the native OCaml runtime, this should only be used when no OCaml code will be linked statically
- `no-std`
  * Allows `ocaml` to be used in `#![no_std]` environments like MirageOS


