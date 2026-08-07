use std::{fmt, io};

use reqwest::Error;

//err confirm

#[derive(Debug)]
pub enum ExchangeError {
    WebSocket(tokio_tungstenite::tungstenite::Error),
    Http(Error),
    Json(serde_json5::Error),
    Io(io::Error),
    Timeout,
    Closed,
    NotFindData,
    NotConnected,
    Disconnected,
}

impl fmt::Display for ExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebSocket(e) => write!(f, "WebSocket error: {e}"),
            Self::Http(e) => write!(f, "Http error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Timeout => write!(f, "Timeout"),
            Self::Closed => write!(f, "Connection closed"),
            Self::NotFindData => write!(f, "Not find data"),
            Self::NotConnected => write!(f, "Not connected"),
            Self::Disconnected => write!(f, "Disconnected"),
        }
    }
}

impl std::error::Error for ExchangeError {}
