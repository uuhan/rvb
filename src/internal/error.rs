use std::result::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum V8Error {
    V8CastErr,
    V8EmptyMaybeLocalErr,
    V8ScriptCompileErr,
    V8ScriptRunErr,
}

impl fmt::Display for V8Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            V8Error::V8CastErr => write!(fmt, "V8 Cast Error."),
            V8Error::V8EmptyMaybeLocalErr => write!(fmt, "Empty MaybeLocal."),
            V8Error::V8ScriptCompileErr => write!(fmt, "Script Compile Failed."),
            V8Error::V8ScriptRunErr => write!(fmt, "Script Run Failed."),
        }
    }
}

impl Error for V8Error {
    fn source(&self) -> Option<&(dyn Error +'static)> {
        match *self {
            // TODO: V8Error
            _ => None,
        }
    }
}

pub type V8Result<T> = Result<T, V8Error>;
