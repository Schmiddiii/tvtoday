use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum Error {
    Networking,
    ParsingWebsite,
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
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(_error: reqwest::Error) -> Self {
        Error::Networking
    }
}
