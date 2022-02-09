use crate::DokiError;
use num::{bigint::TryFromBigIntError, BigInt};
use rust_decimal::Error;

impl From<TryFromBigIntError<BigInt>> for DokiError {
    fn from(e: TryFromBigIntError<BigInt>) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}
