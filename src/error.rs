use value::Value;

/// Error returned by `ocaml-rs` functions
#[derive(Debug)]
pub enum Error {
    /// An index is out of bounds
    OutOfBounds,

    /// A value cannot be called using callback functions
    NotCallable,

    /// An OCaml exception
    Exception(Value),

    /// Array is not a double array
    NotDoubleArray,

    /// C String is invalid
    InvalidCString
}
