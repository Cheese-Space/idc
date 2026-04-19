//!# idc: A simple crate for error propagation  
//!Idc is a simple crate for propagating errors that implement std’s Error trait.  
//!Idc also supports no_std with the same functionality, but you have to provide a global allocator and disable default features.  
//!  
//!## examples:  
//!1. propagating multiple different errors:  
//!  
//!```ignore
//!use std::fs;
//!use idc::*;
//!use serde_json::Value;
//!
//!fn main() -> Result<()> {
//!    let foo = fs::read_to_string("foo.json").context("failed to read foo.", Some("maybe it doesn't exist?"))?;
//!    let json: Value = serde_json::from_str(&foo).context("failed to turn foo into json.", Some("make sure foo.json is valid json."))?;
//!    println!("{}", json["important item"]);
//!    Ok(())
//!}
//!
//!```
//!  
//!2. returning an one-time error:  
//!  
//!```no_run
//!use std::env;
//!use idc::*;
//!
//!fn main() -> Result<()> {
//!    let args: Vec<String> = env::args().collect();
//!    if args.len() < 2 {
//!        bail!("no argument provided!");
//!    }
//!    //...
//!    Ok(())
//!}
//!```
#![no_std]
#![deny(missing_docs)]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(feature = "std")]
use std::string::ToString;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(not(feature = "std"))]
use core::error::Error as StdError;
#[cfg(feature = "std")]
use std::error::Error as StdError;
use core::fmt;

///An error type for errors that implement std's Error trait.  
/// 
///See top-level documentation for usage examples.
pub struct Error {
    msg: String,
    context: Option<String>,
    hint: Option<String>
}
impl Error {
    ///Create a new Error without context.  
    ///You'll probaly never need to use this function.  
    ///For one time errors, see the [`bail!()`] macro.  
    /// 
    ///## example:  
    ///```  
    ///use idc::*;  
    ///fn main() -> Result<()> {  
    ///    let err = Error::new("I failed");
    ///    assert_eq!(&format!("{}", err), "I failed");  
    ///    Ok(())  
    ///}  
    ///```  
    pub fn new(msg: &str) -> Self {
        Error {
            msg: msg.to_string(),
            context: None,
            hint: None
        }
    }
}
impl<E: StdError> From<E> for Error {
    fn from(value: E) -> Self {
        Self {
            msg: value.to_string(),
            context: None,
            hint: None
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(context) = &self.context {
            if let Some(hint) = &self.hint {
                write!(f, "{context}\n\nCaused by:\n\t{}\n\nHint: {hint}", self.msg)
            }
            else {
                write!(f, "{context}\n\nCaused by:\n\t{}", self.msg)
            }
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
///Shoter version of Result<T, idc::Error>.  
/// 
///You can use it the same way as you would use [`std::result::Result<T, E>`].
pub type Result<T, E = Error> = core::result::Result<T, E>;
///Trait for adding context and an optional hint to errors.  
/// 
///## example:  
///```no_run  
///use idc::*;  
///use std::fs;  
///fn main() -> Result<()> {  
///    let foo = fs::read_to_string("foo").context("failed to read foo.", Some("maybe it doesn't exist?"))?;  
///    println!("{}", foo);  
///    Ok(())  
///}
pub trait Context<T, E: StdError> {
    ///Function for adding context and an option hint to an error.  
    ///
    ///See [`Context`] for an exmaple.
    fn context(self, context: &str, hint: Option<&str>) -> Result<T>;
}
impl<T, E: StdError> Context<T, E> for core::result::Result<T, E> {
    fn context(self, context: &str, hint: Option<&str>) -> Result<T> {
        self.map_err(|e| {
            let mut error = Error::from(e);
            error.context = Some(context.to_string());
            error.hint = hint.map(|hint| hint.to_string());
            error
        })
    }
}
#[macro_export]
///Macro for one time errors.  
/// 
///## example:  
///```no_run
///use std::env;
///use idc::*;
///
///fn main() -> Result<()> {
///    let args: Vec<String> = env::args().collect();
///    if args.len() < 2 {
///        bail!("no argument provided!");
///    }
///    //...
///    Ok(())
///}
///```
macro_rules! bail {
    ($($arg:tt)*) => {
        extern crate alloc;
        return Err($crate::Error::new(&alloc::format!($($arg)*)));
    };
}