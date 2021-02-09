use std::fmt::{self, Display};
use std::error::Error;

#[derive(Debug)]
pub enum WasrusError {
    InvalidWasmFileError,
}

impl Display for WasrusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WasrusError::*;
        match self {
            InvalidWasmFileError => write!(f, "Expected wasm binary file, but it's not wasm format"),
        }
    }
}

impl Error for WasrusError {}
