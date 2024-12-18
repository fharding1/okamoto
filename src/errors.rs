use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProveError {
    InvalidDimensions,
    Unsound,
}

impl Display for ProveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProveError::InvalidDimensions => write!(f, "matrix length should be statement length times witness length"),
            ProveError::Unsound => write!(f, "statement is not true for the provided witness, if you really want to proceed then turn off check_soundness feature"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VerifyingError {
    Malformed,
    Invalid,
}

impl Display for VerifyingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VerifyingError::Malformed => write!(f, "proof does not have the correct size"),
            VerifyingError::Invalid => write!(f, "proof is not valid"),
        }
    }
}
