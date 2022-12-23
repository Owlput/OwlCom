use std::{fmt::Display, io};


#[derive(Debug)]
pub enum Error {
    /// A wrapper around `reqwest::Error`.
    Reqwest(reqwest::Error),
    /// A wrapper around `io::Error`.
    Io(io::Error),
    /// A wrapper around `serde_json::Error`.
    SerdeJson(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Io(e) => write!(f, "{}", e),
            Error::SerdeJson(e) => write!(f, "{}", e),
        }
    }
}
