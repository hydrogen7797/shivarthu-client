use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use thiserror::Error;

// https://docs.rs/thiserror/latest/thiserror/

// use thiserror::Error;
// use std::io;

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader {
//         expected: String,
//         found: String,
//     },
//     #[error("unknown data store error")]
//     Unknown,
// }

// fn main() {
//     // Example 1: Creating a Disconnect variant from an io::Error
//     let io_error = io::Error::new(io::ErrorKind::Other, "Custom IO error");
//     let disconnect_error: DataStoreError = DataStoreError::Disconnect(io_error);
//     println!("Disconnect Error: {:?}", disconnect_error);

//     // Example 2: Creating a Redaction variant with a custom message
//     let redaction_error: DataStoreError = DataStoreError::Redaction("some_key".to_string());
//     println!("Redaction Error: {:?}", redaction_error);

//     // Example 3: Creating an InvalidHeader variant with expected and found values
//     let invalid_header_error: DataStoreError = DataStoreError::InvalidHeader {
//         expected: "Header1".to_string(),
//         found: "Header2".to_string(),
//     };
//     println!("Invalid Header Error: {:?}", invalid_header_error);

//     // Example 4: Creating an Unknown variant
//     let unknown_error: DataStoreError = DataStoreError::Unknown;
//     println!("Unknown Error: {:?}", unknown_error);
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorString(pub String);

impl fmt::Display for ErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorString {}
