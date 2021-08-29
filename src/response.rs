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

impl Message {
    /// to_string returns a string representation of the message
    /// so we can print it in the console.
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.user, self.text)
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Image {
//     data: (),
//     // datetime: std::time::Instant,
// }
