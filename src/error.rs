use std::fmt::{Display, Formatter, Result};
use std::io;

#[derive(Clone, Debug)]
pub enum Error {
    Networking,
    ParsingWebsite,
    ParsingFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::Networking => write!(
                f,
                "A networking error occured. Are you connected to the internet?"
            ),
            Error::ParsingWebsite => {
                write!(f, "Could not parse the website. Maybe it has changed?")
            }
            Error::ParsingFile => {
                write!(f, "Could not parse the file about the filters.")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(_error: reqwest::Error) -> Self {
        Error::Networking
    }
}

impl From<io::Error> for Error {
    fn from(_error: io::Error) -> Self {
        Error::ParsingFile
    }
}

impl From<csv::Error> for Error {
    fn from(_error: csv::Error) -> Self {
        Error::ParsingFile
    }
}
