use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    NoTag
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoTag => write!(f, "Your repository does not seem to contain any tag. Please use the release option if you wish to generate a changelog either way.")
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoTag => "Your repository does not seem to contain any tag. Please use the release option if you wish to generate a changelog either way."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
