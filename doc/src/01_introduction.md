[ocaml-rs](https://github.com/zshipko/ocaml-rs) is a Rust crate for interacting with the OCaml runtime. It allows you to write functions in Rust that can be called from OCaml and vice-versa. `ocaml-rs` also does automatic conversion between OCaml and Rust representations.

There are several crates that make this possible:

* [ocaml-sys](https://crates.io/crates/ocaml-sys) - Low level bindings to the OCaml runtime
* [ocaml-boxroot-sys](https://crates.io/crates/ocaml-boxroot-sys) - Bindings to [ocaml-boxroot](https://gitlab.com/ocaml-rust/ocaml-boxroot/), which handles safe allocation of OCaml values
* [ocaml-interop](https://crates.io/crates/ocaml-interop) - Interactions with the OCaml runtime
* [ocaml-derive](https://crates.io/crates/ocaml-derive) - Procedural macros: `ocaml::func`, `ocaml::sig`, `derive(FromValue)`, `derive(ToValue)`
* [ocaml-build](https://crates.io/crates/ocaml-build) - Generate OCaml interfaces from `ocaml::sig` definitions
* [ocaml](https://crates.io/crates/ocaml) - Higher level bindings built using the crates listed above

## Initial setup

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

If you plan on using `ocaml-build`:

```toml
[build-depdencies]
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

It can take a little trial and error to get this right depending on the specifics of your project!

To simplify the full setup process, take a look at [ocaml-rust-starter](https://github.com/zshipko/ocaml-rust-starter).

## Writing your first `ocaml::func`

`ocaml::func` is the highest-level macro that can be used to generate OCaml functions. It's built on `ocaml::native_func` which only works on `Value` parameters and `ocaml::bytecode_func` which is used for generating bytecode functions. `ocaml::func` will take care of generating bytecode bindings for functions with more than five parameters as required by the OCaml runtime.

All `ocaml::func`'s have an implicit `gc` variable which is used to access the OCaml runtime. To make this explicit you can define the name by providing it as an argument to the `ocaml::func` macro:

```rust,ignore
#[ocaml::func(my_gc_name)]
...
```

The following example will read a file and return the contents, we will ignore error handling for now since that will be covered later - however, one thing worth mentioning now is that Rust panics will be converted into OCaml exceptions.

```rust
# extern crate ocaml;

#[ocaml::func]   // This is needed to make the function compatible with OCaml
#[ocaml::sig("string -> string")] /// This is used to generate the OCaml bindings
pub unsafe fn read_file(filename: String) -> String {
  std::fs::read_to_string(filename).unwrap()
}
```

In the above example, automatic conversion is performed between OCaml strings and Rust strings. The next section will provide a table of valid conversions before getting into more details about writing functions and calling OCaml functions from Rust.
