# Linking an OCaml library into a Rust program

The section will cover how to create a program in Rust that calls functions from an OCaml library.

Example project layout:

- Cargo.toml
- build.rs
- `src`: contains Rust code
- `lib`: contains OCaml code and `dune` file
- `dune-project`
- `example.opam`

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ocaml = "*"

[build-dependencies]
ocaml-build = {version = "*", features=["dune"]}
```

And add a `build.rs`, this example assumes your OCaml library and the following `dune` file are in `lib` at the root of your Rust project:

```rust,ignore
# extern crate ocaml_build;
pub fn main() {
  ocaml_build::Dune::new("lib").build()
}
```

If the `dune` root is not the root of your project you can use `Dune::with_root` to set the correct path.

Next you will need to add setup a [dune](https://dune.build) project to handle compilation of your OCaml code. Here is an example `dune` file that will generate object files that can be linked into your Rust program, in this case the OCaml library is named `example`:

```ignore
(executable
 (name example)
 (public_name example)
 (modes exe object))
```

NOTE: The OCaml code needs to be built with dune in `object` mode

In `lib/example.ml`

```ocaml
let hello_world () = "Hello, world"
let () = Callback.register "hello_world" hello_world
```

In `lib/main.rs`:

```rust,ignore
# extern crate ocaml;

ocaml::import! {
  fn hello_world() -> String;
}

pub fn main() {
  let gc = ocaml::init(); // Initialize OCaml runtime

  let s = unsafe { hello_world(&gc).unwrap() };
  println!("{s}");
}
```

NOTE: [ocaml::init](https://docs.rs/ocaml/latest/ocaml/runtime/fn.init.html) needs to be called before attempting to access any OCaml functions.

To simplify the full setup process, take a look at [rust-ocaml-starter](https://github.com/zshipko/rust-ocaml-starter).

