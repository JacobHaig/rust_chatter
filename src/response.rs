use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Message(Vec<Message>),
    // Image(Image),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub text: String,
    // datetime: std::time::Instant,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.user, self.text)
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Image {
//     data: (),
//     // datetime: std::time::Instant,
// }
