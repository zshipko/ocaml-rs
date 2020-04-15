use crate::{FromValue, ToValue, Value};

/// Errors that are translated directly into OCaml exceptions
#[derive(Debug)]
pub enum CamlError {
    /// Not_found
    NotFound,

    /// Failure
    Failure(String),

    /// Invalid_argument
    InvalidArgument(String),

    /// Out_of_memory
    OutOfMemory,

    /// Stack_overflow
    StackOverflow,

    /// Sys_error
    SysError(String),

    /// End_of_file
    EndOfFile,

    /// Zero_divide
    ZeroDivide,

    /// Array bound error
    ArrayBoundError,

    /// Sys_blocked_io
    SysBlockedIo,

    /// A pre-allocated OCaml exception
    Exception(Value),

    /// An exception type and argument
    WithArg(Value, Value),
}

/// Error returned by `ocaml-rs` functions
#[derive(Debug)]
pub enum Error {
    /// A value cannot be called using callback functions
    NotCallable,

    /// Array is not a double array
    NotDoubleArray,

    /// Error message
    Message(String),

    /// General error
    Error(Box<dyn std::error::Error>),

    /// OCaml exceptions
    Caml(CamlError),
}

impl<T: 'static + std::error::Error> From<T> for Error {
    fn from(x: T) -> Error {
        Error::Error(Box::new(x))
    }
}

impl From<CamlError> for Error {
    fn from(x: CamlError) -> Error {
        Error::Caml(x)
    }
}

impl Error {
    /// Re-raise an existing exception value
    pub fn reraise(exc: Value) -> Result<(), Error> {
        Err(CamlError::Exception(exc).into())
    }

    /// Raise an exception that has been registered using `Callback.register_exception` with no
    /// arguments
    pub fn raise<S: AsRef<str>>(name: S) -> Result<(), Error> {
        let s = Self::named(name.as_ref()).unwrap_or_else(|| {
            panic!(
                "{} has not been registered as an exception with OCaml",
                name.as_ref()
            )
        });
        Err(CamlError::Exception(s).into())
    }

    /// Raise an exception that has been registered using `Callback.register_exception` with an
    /// argument
    pub fn raise_with_arg<S: AsRef<str>, T: ToValue>(name: S, arg: T) -> Result<(), Error> {
        let s = Self::named(name.as_ref()).unwrap_or_else(|| {
            panic!(
                "{} has not been registered as an exception with OCaml",
                name.as_ref()
            )
        });
        Err(CamlError::WithArg(s, arg.to_value()).into())
    }

    /// Raise `Not_found`
    pub fn not_found() -> Result<(), Error> {
        Err(CamlError::NotFound.into())
    }

    /// Raise `Out_of_memory`
    pub fn out_of_memory() -> Result<(), Error> {
        Err(CamlError::OutOfMemory.into())
    }

    /// Raise `Failure`
    pub fn failwith<S: AsRef<str>>(s: S) -> Result<(), Error> {
        Err(CamlError::Failure(s.as_ref().into()).into())
    }

    /// Raise `Invalid_argument`
    pub fn invalid_argument<S: AsRef<str>>(s: S) -> Result<(), Error> {
        Err(CamlError::Failure(s.as_ref().into()).into())
    }

    /// Get named error registered using `Callback.register_exception`
    pub fn named<S: AsRef<str>>(s: S) -> Option<Value> {
        Value::named(s.as_ref())
    }
}

unsafe impl<T: ToValue, E: std::error::Error> ToValue for Result<T, E> {
    fn to_value(self) -> Value {
        match self {
            Ok(x) => x.to_value(),
            Err(y) => {
                let e: Result<T, Error> = Err(Error::Message(format!("{:?}", y)));
                e.to_value()
            }
        }
    }
}

unsafe impl<T: ToValue> ToValue for Result<T, Error> {
    fn to_value(self) -> Value {
        match self {
            Ok(x) => return x.to_value(),
            Err(Error::Caml(CamlError::Exception(e))) => unsafe {
                crate::sys::caml_raise(e.0);
            },
            Err(Error::Caml(CamlError::NotFound)) => unsafe {
                crate::sys::caml_raise_not_found();
            },
            Err(Error::Caml(CamlError::ArrayBoundError)) => unsafe {
                crate::sys::caml_array_bound_error();
            },
            Err(Error::Caml(CamlError::OutOfMemory)) => unsafe {
                crate::sys::caml_array_bound_error();
            },
            Err(Error::Caml(CamlError::EndOfFile)) => unsafe {
                crate::sys::caml_raise_end_of_file()
            },
            Err(Error::Caml(CamlError::StackOverflow)) => unsafe {
                crate::sys::caml_raise_stack_overflow()
            },
            Err(Error::Caml(CamlError::ZeroDivide)) => unsafe {
                crate::sys::caml_raise_zero_divide()
            },
            Err(Error::Caml(CamlError::SysBlockedIo)) => unsafe {
                crate::sys::caml_raise_sys_blocked_io()
            },
            Err(Error::Caml(CamlError::InvalidArgument(s))) => {
                unsafe {
                    let s = std::ffi::CString::new(s.as_bytes()).expect("Invalid C string");
                    crate::sys::caml_invalid_argument(s.as_ptr() as *const std::os::raw::c_char)
                };
            }
            Err(Error::Caml(CamlError::WithArg(a, b))) => unsafe {
                crate::sys::caml_raise_with_arg(a.0, b.0)
            },
            Err(Error::Caml(CamlError::SysError(s))) => {
                unsafe {
                    let s = s.to_value();
                    crate::sys::caml_raise_sys_error(s.0)
                };
            }
            Err(Error::Message(s)) | Err(Error::Caml(CamlError::Failure(s))) => {
                unsafe {
                    let s = std::ffi::CString::new(s.as_bytes()).expect("Invalid C string");
                    crate::sys::caml_failwith(s.as_ptr() as *const std::os::raw::c_char)
                };
            }
            Err(Error::Error(e)) => {
                let s = format!("{:?}\0", e);
                unsafe { crate::sys::caml_failwith(s.as_ptr() as *const std::os::raw::c_char) };
            }
            Err(Error::NotDoubleArray) => {
                let s = "invalid double array\0";
                unsafe { crate::sys::caml_failwith(s.as_ptr() as *const std::os::raw::c_char) };
            }
            Err(Error::NotCallable) => {
                let s = "value is not callable\0";
                unsafe { crate::sys::caml_failwith(s.as_ptr() as *const std::os::raw::c_char) };
            }
        };

        Value::unit()
    }
}

unsafe impl<T: FromValue> FromValue for Result<T, crate::Error> {
    fn from_value(value: Value) -> Result<T, crate::Error> {
        if value.is_exception_result() {
            return Err(CamlError::Exception(value.exception().unwrap()).into());
        }

        Ok(T::from_value(value))
    }
}
