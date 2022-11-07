#[derive(Debug)]
pub enum Error {
    HTTPInternalError(String),
    GenericError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::GenericError(resp) => write!(f, "Generic Error: {:?}", resp),
            Error::HTTPInternalError(resp) => write!(f, "HTTP Request failed: {:?}", resp),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPInternalError(e.to_string())
    }
}
