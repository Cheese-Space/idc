#![no_std]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(not(feature = "std"))]
use core::error::Error as StdError;
#[cfg(feature = "std")]
use std::error::Error as StdError;

pub struct Error {
    msg: String,
    context: Option<String>
}
pub type Result<T, E = Error> = core::result::Result<T, E>;
pub trait Context<T> {
    fn context(self) -> Result<T>;
}