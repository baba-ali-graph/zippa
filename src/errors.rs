use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ZippaError {
    InvalidCompressionMethod,
}

impl std::error::Error for ZippaError {}

impl Display for ZippaError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ZippaError::InvalidCompressionMethod => write!(f, "Invalid compression method passed"),
        }
    }
}
