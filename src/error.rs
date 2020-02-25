#[derive(Debug)]
pub enum Error {
    IndexOutOfRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IndexOutOfRange => write!(f, "Index out of range"),
        }
    }
}

impl std::error::Error for Error {}
