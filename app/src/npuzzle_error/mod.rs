use std::fmt;
use std::error;

// type Result<T> = std::result::Result<T, NPuzzleError>;
#[derive(Debug)]
#[derive(PartialEq)]
pub enum NPuzzleError {
    STDError,
    WrongPuzzleHeight,
    WrongPuzzleWidth,
    WrongPuzzleSize,
    InvalidNumberInPuzzle
}

impl fmt::Display for NPuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NPuzzleError::STDError =>
                write!(f, "Npuzzle error"),
            NPuzzleError::WrongPuzzleHeight =>
                write!(f, "Size provided doesn't match the puzzle height"),
            NPuzzleError::WrongPuzzleWidth =>
                write!(f, "Size provided doesn't match the puzzle width"),
            NPuzzleError::WrongPuzzleSize =>
                write!(f, "Size provided doesn't match the puzzle size"),
            NPuzzleError::InvalidNumberInPuzzle =>
                write!(f, "Invalid number in puzzle")
        }
    }
}

impl error::Error for NPuzzleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NPuzzleError::WrongPuzzleHeight => None,
            NPuzzleError::WrongPuzzleWidth => None,
            NPuzzleError::WrongPuzzleSize => None,
            NPuzzleError::InvalidNumberInPuzzle => None,
            NPuzzleError::STDError => None
        }
    }
}

impl From<std::io::Error> for NPuzzleError {
    fn from(_e: std::io::Error) -> NPuzzleError {
        NPuzzleError::STDError
    }
}