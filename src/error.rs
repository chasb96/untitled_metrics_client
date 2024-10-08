use std::fmt::Display;

use prost::DecodeError;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Decode(DecodeError),
    Status(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_status() {
            Self::from(value.status().unwrap())
        } else {
            Self::Http(value)
        }
    }
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        Self::Status(value)
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(http) => write!(f, "Error calling auth host: {}", http),
            Error::Decode(decode) => write!(f, "Error decoding response: {}", decode),
            Error::Status(status) => write!(f, "Auth host returned an error: {}", status),
        }
    }
}

impl std::error::Error for Error { }