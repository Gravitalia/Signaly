use serde::{Deserialize, Serialize};

/// Cloudevents structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event
{
    /// CloudEvents specification version.
    pub specversion: String,
    /// Type of the event.
    pub r#type: String,
    /// Source of the event.
    pub source: String,
    /// Unique identifier for the event.
    /// Will be used to save in database.
    pub id: String,
    /// Timestamp of when the event occurred in RFC 3339 format.
    pub time: String,
    /// Must be application/json.
    pub datacontenttype: String,
    /// Data associated with the event.
    pub data: Data,
}

/// Data transmitted by the broker.
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    /// Vanity of the user who sanctioned or reported the incident.
    pub from: String,
    /// Vanity of the user affected by the sanction or report.
    pub to: String,
    /// Reason for the sanction or warning to be recorded. 
    pub reason: Reason,
    /// Defines message processing.
    #[serde(default)]
    pub r#type: Type,
    /// Sanction taken against user.
    pub sanction: Option<Sanction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Report,
    Sanction,
}

impl Default for Type {
    fn default() -> Self {
        Self::Report
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum Reason {
    Copyright = 0,
    Defamation = 1,
    Hate = 2,
    Harassment = 3,
    Nudity = 4,
    Spam = 5,
    Violence = 6,
    Other(String) = 7,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum Sanction {
    /// The user no longer has access to services.
    Suspension = 0,
    /// Content (publication, comment, etc.) is permanently removed.
    Removal = 1,
}
