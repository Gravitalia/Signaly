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

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GravitaliaPost {
    id: String,
    hash: Vec<String>,
    description: String,
    text: String,
    pub like: usize,
    pub author: String
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AuthaUser {
    username: String,
    pub vanity: String,
    avatar: Option<String>,
    bio: Option<String>,
    email: Option<String>,
    birthdate: Option<String>,
    verified: bool,
    deleted: bool,
    pub flags: u32
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub services: Vec<String>,
}

#[derive(Deserialize)]
pub struct Suspend {
    pub vanity: String,
    pub platform: String
}
