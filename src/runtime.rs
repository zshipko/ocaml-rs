use crate::Runtime;

/// Initialize the OCaml runtime, the runtime will be
/// freed when the value goes out of scope
pub fn init() -> Runtime {
    Runtime::init()
}

/// Initialize the OCaml runtime
pub fn init_persistent() {
    Runtime::init_persistent()
}
