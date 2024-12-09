use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseErrors {
    #[error(transparent)]
    ParseLiteralError(#[from] ParseLiteralError),
}

#[derive(Error, Debug)]
pub enum ParseLiteralError {
    #[error(transparent)]
    ParseIntegerError(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}
