use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32, // Optional -- This gets automatically set by the database
    pub username: String,
    pub content: String,
    pub timestamp: i64, // Optional -- This gets automatically set by the database
}

impl Message {
    /// Creates a new message
    pub fn new(username: String, content: String) -> Message {
        Message {
            id: 0,
            username,
            content,
            timestamp: 0,
        }
    }

    // Get the current time from the timestamp
    pub fn get_time(&self) -> String {
        let time = chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0);
        time.to_string()
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
