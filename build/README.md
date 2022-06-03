# ocaml-build

`ocaml-build` is used to generate an OCaml file containing signatures from Rust code

For example, if you have this function (annotated with the `#[ocaml::sig(...)]` macro:

```rust
#[ocaml::func]
#[ocaml::sig("int -> bool")]
pub fn greater_than_zero(i: ocaml::Int) -> bool {
  i > 0
}
```

And the following build script:

```rust
fn main() -> std::io::Result<()> {
    ocaml_build::Sigs::new("src/rust.ml").generate()
}
```

The following code will be generated in `src/rust.ml`:

```ocaml
external greater_than_zero: int -> bool = "greater_than_zero"
```

And a matching `mli` file will be created.
