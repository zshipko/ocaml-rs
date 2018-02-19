/// Error returned by `ocaml-rs` functions
#[derive(Debug)]
pub enum Error {
    /// An index is out of bounds
    OutOfBounds,

    /// A value cannot be called using callback functions
    NotCallable
}
