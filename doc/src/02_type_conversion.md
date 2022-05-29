# Type conversion

As mentioned in the previous section, `ocaml-rs` automates the conversion between Rust and OCaml representations for many types. This is done using two traits: `ToValue`, which is implemented for types that can be converted to an OCaml value and `FromValue` for types that can be converted from an OCaml value.

Below is a list of types that implement these traits in `ocaml-rs` and their corresponding OCaml type:

| Rust type                 | OCaml type           |
| ------------------------- | -------------------- |
| `()`                      | `unit`               |
| `isize`                   | `int`                |
| `usize`                   | `int`                |
| `i8`                      | `int`                |
| `u8`                      | `int`                |
| `i16`                     | `int`                |
| `u16`                     | `int`                |
| `i32`                     | `int32`              |
| `u32`                     | `int32`              |
| `i64`                     | `int64`              |
| `u64`                     | `int64`              |
| `f32`                     | `float`              |
| `f64`                     | `float`              |
| `str`                     | `string`             |
| `[u8]`                    | `bytes`              |
| `String`                  | `string`             |
| `Option<A>`               | `'a option`          |
| `Result<A, ocaml::Error>` | `'a` or `exception`  |
| `Result<A, B>`            | `('a, 'b) Result.t`  |
| `(A, B, C)`               | `'a * 'b * 'c`       |
| `&[Value]`                | `'a array` (no copy) |
| `Vec<A>`, `&[A]`          | `'a array`           |
| `BTreeMap<A, B>`          | `('a, 'b) list`      |
| `LinkedList<A>`           | `'a list`            |

NOTE: Even though `&[Value]` is specifically marked as no copy, any type like `Option<Value>` would also qualify since the inner value is not converted to a Rust type. However, `Option<String>` will do full unmarshaling into Rust types. Another thing to note: `FromValue` for `str` and `&[u8]` is zero-copy, however `ToValue` for `str` and `&[u8]` creates a new value - this is necessary to ensure the string is registered with the OCaml runtime.

If you're concerned with minimizing allocations/conversions you should use `Value` type directly.

## Implementing `ToValue` and `FromValue`

The `ToValue` trait has a single function, `to_value`, that takes a `Value` and returns the new type and `FromValue` takes a reference to a type and returns a new `Value`:

```rust
# extern crate ocaml;

pub struct MyType(i32);

unsafe impl ocaml::ToValue for MyType {
  fn to_value(&self, _gc: &ocaml::Runtime) -> ocaml::Value {
    unsafe { ocaml::Value::int32(self.0) }
  }
}

unsafe impl ocaml::FromValue for MyType {
  fn from_value(value: ocaml::Value) -> MyType {
    unsafe { MyType(value.int32_val()) }
  }
}
```

This can also be accomplished using the derive macros:

```rust
# extern crate ocaml;

#[derive(ocaml::ToValue, ocaml::FromValue)]
pub struct MyType(i32);
```

`derive(ToValue, FromValue)` will work on any struct or enum that are comprised of types that also implement `ToValue` and `FromValue`

## Types that work directly on OCaml values

There are several types that work directly on OCaml values, these don't perform any copies when converting to and from `Value`.

| Rust type                      | OCaml type
| ------------------------------ | -------------------------------------- |
| `ocaml::Array<T>`              | `'a array`                             |
| `ocaml::List<T>`               | `'a list`                              |
| `ocaml::bigarray::Array1<T>`   | `('a, 'b, c_layout) Bigarray.Array1.t` |
| `ocaml::bigarray::Array2<T>`   | `('a, 'b, c_layout) Bigarray.Array2.t` |
| `ocaml::bigarray::Array3<T>`   | `('a, 'b, c_layout) Bigarray.Array3.t` |

## Wrapping Rust values

Rust values can be used as opaque values that can be shared with OCaml using `ocaml::Pointer<T>`. The `Pointer` type allows for Rust values to be allocated using the OCaml runtime, this means their lifetime will be handled by the garbage collector. `Pointer::alloc_final` is used to move an existing Rust type into an OCaml allocated pointer, but even better is the option to implement the `Custom` trait for your type.

Implementing `Custom` allows you to define equality/comparison, finalization, hashing and serialization functions for your type that will be used by OCaml. When allocation custom values you should use `Pointer::alloc_custom`.

In either case you will need to write the allocation function in Rust because OCaml doesn't know the specifics about the layout or contents of these types, unlike when using `FromValue` or `ToValue`. `Pointer` should primarily be used on Rust values that cannot be converted directly to OCaml types.

```rust
# extern crate ocaml;

#[ocaml::sig("")]   // Creates an opaque type on the OCaml side
pub struct MyType {
  a: i32,
  b: f64,
  c: std::fs::File, // This can't be converted to an OCaml value
}

extern "C" fn mytype_finalizer(_: ocaml::Raw) {
  println!("This runs when the value gets garbage collected");
}

ocaml::custom!(MyType {
  finalize: mytype_finalizer
});

#[ocaml::func]
#[ocaml::sig("my_type -> float")]
pub unsafe fn my_type_add_a_b(t: ocaml::Pointer<MyType>) -> f64 {
  let t = t.as_ref();
  t.a as f64 + t.b
}
```

Now that you have some insight into how type conversion is handled, the next section will cover more details about writing OCaml functions in Rust.
