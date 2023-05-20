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

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GravitaliaUser {
    pub followers: u32,
    following: u32,
    public: bool,
    pub suspended: bool,
    access_post: bool
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub services: Vec<String>,
}