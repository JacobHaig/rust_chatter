use std::fmt;

use serde::{Deserialize, Serialize};

use crate::database::MessageRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32, // Optional -- This gets automatically set by the database
    pub username: String,
    pub content: String,
    pub timestamp_ms: i64, // Optional -- This gets automatically set by the database
}

impl Message {
    /// Creates a new message
    pub fn new(username: String, content: String) -> Message {
        Message {
            id: 0,
            username,
            content,
            timestamp_ms: 0,
        }
    }

    // Get the current time from the timestamp
    pub fn get_time(&self) -> String {
        // TODO: Figure out how to get the local time
        // from the timestamp

        // use chrono::{DateTime, FsixedOffset, Local, Utc};
        // let now = chrono::Local::now();
        // let naive_date = chrono::NaiveDateTime::from_timestamp(self.timestamp_ms / 1000, 0);
        // let local = chrono::DateTime::<chrono::Local>::from_utc(naive_date, );

        // let time = chrono::NaiveDateTime::from_timestamp(self.timestamp_ms / 1000, 0);
        let time = chrono::NaiveDateTime::from_timestamp_opt(self.timestamp_ms / 1000, 0).unwrap();
        time.to_string()
    }

    // Convert to MessageRow
    pub fn to_row(&self) -> MessageRow {
        MessageRow {
            username: Some(self.username.clone()),
            content: Some(self.content.clone()),
            timestamp_ms: Some(self.timestamp_ms),
            ..Default::default()
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {}: {}",
            self.get_time(),
            self.username,
            self.content
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32, // Optional -- This gets automatically set by the database
    pub username: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}
