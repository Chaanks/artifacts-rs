pub mod my_account;
pub mod my_characters;

use my_characters::CharacterMovementData;

use serde::{Deserialize, Serialize};
use std::fmt;

use super::Error;

pub enum ResponseData {
    CharacterMovement(CharacterMovementData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub error: Error,
}

impl ResponseError {
    pub fn new(code: u16, message: String) -> Self {
        let error = Error { code, message };
        Self { error }
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error.message.to_lowercase().replace('.', ""))
    }
}

impl std::error::Error for ResponseError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusData {
    /// Current server status.
    pub status: String,
    /// Current game version.
    pub version: String,
    /// Maximum level a character can reach in the game.
    pub max_level: i32,
    /// Number of characters currently online.
    pub characters_online: i32,
    /// Current server time in ISO 8601 format.
    pub server_time: String,
    /// List of server announcements.
    pub announcements: Vec<AnnouncementSchema>,
    /// Datetime of the last server wipe in ISO 8601 format.
    pub last_wipe: String,
    /// Datetime of the next planned server wipe in ISO 8601 format.
    pub next_wipe: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnouncementSchema {
    /// The announcement message text.
    pub message: String,
    /// Datetime when the announcement was created (optional).
    pub created_at: Option<String>,
}
