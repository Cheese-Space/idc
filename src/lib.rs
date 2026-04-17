#![no_std]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use alloc::string::ToString;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(not(feature = "std"))]
use core::error::Error as StdError;
#[cfg(feature = "std")]
use std::error::Error as StdError;
use core::fmt;
pub struct Error {
    msg: String,
    context: Option<String>
}
impl<E: StdError> From<E> for Error {
    fn from(value: E) -> Self {
        Self {
            msg: value.to_string(),
            context: None
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(context) = &self.context {
            write!(f, "{context}\n\nCaused by:\n\t{}", self.msg)
        }
        else {
            write!(f, "{}", self.msg)
        }
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
pub type Result<T, E = Error> = core::result::Result<T, E>;
pub trait Context<T, E: StdError> {
    fn context(self, context: &str) -> Result<T>;
}
impl<T, E: StdError> Context<T, E> for core::result::Result<T, E> {
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| {
            let mut error = Error::from(e);
            error.context = Some(context.to_string());
            error
        })
    }
}
