use std::fmt;
use std::error;

// type Result<T> = std::result::Result<T, NPuzzleError>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NPuzzleError {
    BadSizeInput,
    IO
}

impl fmt::Display for NPuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NPuzzleError::BadSizeInput =>
                write!(f, "Size provided doesn't match the puzzle size"),
            _ => unimplemented!()
        }
    }
}

impl error::Error for NPuzzleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NPuzzleError::BadSizeInput => None,
            _ => unimplemented!()
        }
    }
}

impl From<std::io::Error> for NPuzzleError {
    fn from(_e: std::io::Error) -> NPuzzleError {
        NPuzzleError::IO
    }
}