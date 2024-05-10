use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub detail: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {} | {}", self.code, self.message, self.detail)
    }
}
