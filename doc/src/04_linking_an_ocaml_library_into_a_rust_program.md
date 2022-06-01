# Linking an OCaml library into a Rust program

The section will cover how to create a program in Rust that calls functions from an OCaml library.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ocaml = "*"

[build-dependecies]
ocaml-build = {version = "*", features=["dune"]}
```

And add a `build.rs`, this example assumes your OCaml library and the following `dune` file are in `lib` at the root of your Rust project:

```rust,ignore
# extern crate ocaml_build;
pub fn main() {
  ocaml_build::Dune::new("lib").build()
}
```

Next you will need to add setup a [dune](https://dune.build) project to handle compilation of your OCaml code. Here is an example `dune` file that will generate object files that can be linked into your Rust program, in this case the OCaml library is named `example`:

```ignore
(executable
 (name example)
 (public_name example)
 (modes exe object))
```

NOTE: The OCaml code needs to be built with dune in `object` mode

To simplify the full setup process, take a look at [rust-ocaml-starter](https://github.com/zshipko/rust-ocaml-starter).

