use std::{fmt::Display, io};

#[derive(Debug)]
pub struct Error {
    kind:Box<Kind>
}
impl Error {
    pub fn new(kind: Kind) -> Self {
        Self { kind: Box::new(kind)}
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Kind {
    Reqwest(reqwest::Error),
    Fs(io::Error),
    SerdeJson(serde_json::Error),
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Reqwest(e) => write!(f, "{}", e),
            Kind::Fs(e) => write!(f, "{}", e),
            Kind::SerdeJson(e) => write!(f, "{}", e),
        }
    }
}
