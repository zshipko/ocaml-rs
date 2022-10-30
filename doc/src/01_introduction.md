[ocaml-rs](https://github.com/zshipko/ocaml-rs) is a Rust crate for interacting with the OCaml runtime. It allows you to write functions in Rust that can be called from OCaml and vice-versa. `ocaml-rs` also does automatic conversion between OCaml and Rust representations. There are several crates that make this possible:

* [ocaml-sys](https://crates.io/crates/ocaml-sys) - Low level bindings to the OCaml runtime
* [ocaml-boxroot-sys](https://crates.io/crates/ocaml-boxroot-sys) - Bindings to [ocaml-boxroot](https://gitlab.com/ocaml-rust/ocaml-boxroot/), which handles safe allocation of OCaml values
* [ocaml-interop](https://crates.io/crates/ocaml-interop) - Interactions with the OCaml runtime
* [ocaml-derive](https://crates.io/crates/ocaml-derive) - Procedural macros: `ocaml::func`, `ocaml::sig`, `derive(FromValue)`, `derive(ToValue)`
* [ocaml-build](https://crates.io/crates/ocaml-build) - Generate OCaml interfaces from `ocaml::sig` definitions
* [ocaml](https://crates.io/crates/ocaml) - Higher level bindings built using the crates listed above

Before going any further, it may be helpful to read through the [Interfacing C with OCaml](https://v2.ocaml.org/manual/intfc.html) from the OCaml handbook if you haven't already!

## Initial setup

This section will cover how to set up a Rust crate that is linked into an OCaml program. If you're interested in calling into an OCaml library from Rust instead, see [Linking an OCaml library into a Rust program](./04_linking_an_ocaml_library_into_a_rust_program.md).

Add the following to your `Cargo.toml`:

```toml
[lib]
crate-type = ["staticlib"] # You can also use cdylib, depending on your project

[dependencies]
ocaml = "*"
```

Additionally, on macOS you may need to add a `.cargo/config` with the following:

```toml
[build]
rustflags = ["-C", "link-args=-Wl,-undefined,dynamic_lookup"]
```

This is because macOS doesn't allow undefined symbols in dynamic libraries by default.

If you plan on using `ocaml-build`:

```toml
[build-dependencies]
ocaml-build = "*"
```

And add a `build.rs`:

```rust,ignore
# extern crate ocaml_build;
pub fn main() -> std::io::Result<()> {
  ocaml_build::Sigs::new("src/rust.ml").generate()
}
```

This build script will look for usages of `#[ocaml::sig(...)]` to generate OCaml bindings.

Next you will need to add setup a [dune](https://dune.build) project to handle compilation of your OCaml code. Here is an example `dune` file that will link your Rust project, in this case the Rust crate is named `example`:

```ignore
(rule
 (targets libexample.a)
 (deps (glob_files *.rs))
 (action
  (progn
   (run cargo build --target-dir %{project_root}/../../target --release)
   (run mv %{project_root}/../../target/release/libexample.a libexample.a))))

(library
 (name example)
 (public_name example)
 (foreign_archives example)
 (c_library_flags
  (-lpthread -lc -lm)))
```

You should also add the following stanza to a `dune` file at the root of your project to ignore the `target` directory:

```ignore
(dirs :standard \ target)
```

It can take a little trial and error to get this right depending on the specifics of your project!

Additionally, if you plan on releasing to [opam](https://github.com/ocaml/opam), you will need to vendor your Rust dependencies to avoid making network requests during the build phase, since reaching out to crates.io/github will be blocked by the opam sandbox. To do this you should run:

```shell
cargo vendor
```
then follow the instructions for editing `.cargo/config`

To simplify the full setup process, take a look at [ocaml-rust-starter](https://github.com/zshipko/ocaml-rust-starter).

## Build options

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


## Writing your first `ocaml::func`

[ocaml::func](https://docs.rs/ocaml/latest/ocaml/attr.func.html) is the highest-level macro that can be used to generate OCaml functions. It's built on `ocaml::native_func` which only works on `Value` parameters and `ocaml::bytecode_func` which is used for generating bytecode functions. `ocaml::func` will take care of generating bytecode bindings for functions with more than five parameters as required by the OCaml runtime. `ocaml::func` handles using `CAMLparam`/`CAMLlocal`/`CAMLreturn` correctly for you, often making it much easier to write bindings than using the C API directly, particularly for those who haven't used the OCaml C API before.

All `ocaml::func`'s have an implicit `gc` variable which is used to access the OCaml runtime. To pick another name you can provide it as an argument to the `ocaml::func` macro:

```rust,ignore
#[ocaml::func(my_gc_name)]
...
```

The following example will read a file and return the contents, we will ignore error handling for now since that will be covered later - however, one thing worth mentioning is that Rust panics will be converted into OCaml exceptions.

```rust
# extern crate ocaml;

#[ocaml::func]   // This is needed to make the function compatible with OCaml
#[ocaml::sig("string -> string")] /// This is used to generate the OCaml bindings
pub unsafe fn read_file(filename: String) -> String {
  std::fs::read_to_string(filename).unwrap()
}
```

In the above example, automatic conversion is performed between OCaml strings and Rust strings. The next section will provide a table of valid conversions before getting into more details about writing functions and calling OCaml functions from Rust.
