use std::error;
use std::fmt;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::ffi::NulError;

#[derive(Debug)]
pub enum DoogieError {
    NulError(NulError),
    Utf8Error(Utf8Error),
    ReturnCode(u32),
    BadEnum(u32),
    IOError(IOError),
    ResourceUnavailable
}

impl fmt::Display for DoogieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoogieError::NulError(ref err) => write!(f, "NulError: {}", err),
            DoogieError::Utf8Error(ref err) => write!(f, "Utf8Error: {}", err),
            DoogieError::IOError(ref err) => write!(f, "IOError: {}", err),
            DoogieError::ReturnCode(code) => write!(f, "CMark return code: {}", code),
            DoogieError::BadEnum(num) => write!(f, "Bad Enum Value: {}", num),
            DoogieError::ResourceUnavailable => write!(f, "The resource is no longer available")
        }
    }
}

impl error::Error for DoogieError {
    fn description(&self) -> &str {
        match *self {
            DoogieError::NulError(ref err) => err.description(),
            DoogieError::Utf8Error(ref err) => err.description(),
            DoogieError::IOError(ref err) => err.description(),
            DoogieError::ReturnCode(_code) => "libcmark returned bad status code.",
            DoogieError::BadEnum(_num) => "libcmark returned a non-matching enum value.",
            DoogieError::ResourceUnavailable => "The resource is no longer available."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DoogieError::NulError(ref err) => Some(err),
            DoogieError::Utf8Error(ref err) => Some(err),
            DoogieError::IOError(ref err) => Some(err),
            DoogieError::ReturnCode(_code) => None,
            DoogieError::BadEnum(_num) => None,
            DoogieError::ResourceUnavailable => None
        }
    }
}

impl From<NulError> for DoogieError {
    fn from(err: NulError) -> DoogieError {
        DoogieError::NulError(err)
    }
}

impl From<Utf8Error> for DoogieError {
    fn from(err: Utf8Error) -> DoogieError {
        DoogieError::Utf8Error(err)
    }
}

impl From<IOError> for DoogieError {
    fn from(err: IOError) -> DoogieError { DoogieError::IOError(err) }
}
