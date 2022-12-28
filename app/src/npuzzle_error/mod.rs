use std::fmt;
use std::error;

type Result<T> = std::result::Result<T, NPuzzleError>;
#[derive(Debug)]
#[derive(PartialEq)]
pub enum NPuzzleError {
    BadSizeInput,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    // Parse(ParseIntError),
}

impl fmt::Display for NPuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NPuzzleError::BadSizeInput =>
                write!(f, "Size provided doesn't match the puzzle size"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            // NPuzzleError::Parse(..) =>
            //     write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for NPuzzleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NPuzzleError::BadSizeInput => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            // NPuzzleError::Parse(ref e) => Some(e),
        }
    }
}