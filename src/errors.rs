use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProveError {
    InvalidDimensions,
    Unsound,
    Error,
}

impl Display for ProveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*match *self {
            SigningError::CompressedPointFormat => {
                write!(f, "Compressed Ristretto point is incorrectly formatted")
            }
            SigningError::PointDecompression => write!(f, "Cannot decompress Ristretto point"),
            SigningError::ScalarFormat => write!(f, "Scalar is not canonically formatted"),
        }*/
        write!(f, "Error")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VerifyingError {
    Error,
    Invalid,
}

impl Display for VerifyingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*match *self {
            SigningError::CompressedPointFormat => {
                write!(f, "Compressed Ristretto point is incorrectly formatted")
            }
            SigningError::PointDecompression => write!(f, "Cannot decompress Ristretto point"),
            SigningError::ScalarFormat => write!(f, "Scalar is not canonically formatted"),
        }*/
        write!(f, "Error")
    }
}

/*
impl Error for SigningError {}

impl From<TryFromSliceError> for SigningError {
    fn from(_: TryFromSliceError) -> SigningError {
        SigningError::CompressedPointFormat
    }
}
*/
