//! Module for error handling 
use std::result;
use std::fmt;
use std::error;
use std::io;
use std::num;
use std::convert;
use std::time;
use std::str;
use cpython;
#[derive(Debug)]
/// Result type. Mainly for using ? macro.
/// One can expand this struct for any other Error type
/// by editting Display implementation and 
/// adding From trait.
pub enum Error{
    /// Error from file IO.
    IoError(io::Error),
    /// Error from parsing string to f32,i32,etc..
    UTF8Error(str::Utf8Error),
    /// Error from fast5 reader 
    Fast5Error(num::ParseFloatError),
    /// Error from benchmarking system
    SystemTimeError(time::SystemTimeError),
    /// Error from cpython
    PyError(cpython::PyErr),
    /// Error from algorithm(if any)
    AlgError,
    /// Error from something unexpected
    OtherError,
}
impl fmt::Display for Error{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        match *self{
            Error::IoError(ref why) => write!(f,"IO Error:{}",why),
            Error::UTF8Error(ref why) => write!(f,"UTF error:{}",why),
            Error::Fast5Error(ref why) => write!(f,"Convert Error:{}",why),
            Error::SystemTimeError(ref why) => write!(f,"SystemTime Error:{}",why),
            Error::PyError(_) => write!(f,"InnerPython Error"),
            Error::AlgError => write!(f,"Alg error"),
            Error::OtherError => write!(f,"other error"),
        }
    }
}
impl error::Error for Error{
    fn description(&self) -> &str{
        "hai"
    }
    fn cause(&self) -> Option<&error::Error>{
        None
    }
}
impl convert::From<io::Error> for Error{
    fn from(err: io::Error) -> Error{
        Error::IoError(err)
    }
}
impl convert::From<num::ParseFloatError> for Error{
    fn from(err: num::ParseFloatError) -> Error{
        Error::Fast5Error(err)
    }
}
impl convert::From<time::SystemTimeError> for Error{
    fn from(err: time::SystemTimeError) -> Error{
        Error::SystemTimeError(err)
    }
}
impl convert::From<str::Utf8Error> for Error{
    fn from(err : str::Utf8Error) -> Error{
        Error::UTF8Error(err)
    }
}
impl convert::From<cpython::PyErr> for Error{
    fn from(err : cpython::PyErr) -> Error{
        Error::PyError(err)
    }
}
/// Alius for Result type frequentyly used in this crate.
pub type Result<T> = result::Result<T,Error>;
