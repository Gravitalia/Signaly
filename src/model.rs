use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Error {
    pub error: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct Signal {
    pub vanity: String,
    pub platform: String,
    pub reason: u8,
}