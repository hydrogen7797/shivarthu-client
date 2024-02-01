use std::error::Error;
use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorString(pub String);

impl fmt::Display for ErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorString {}