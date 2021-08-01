use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Message(Message),
    Image(Image),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub text: String,
    // datetime: std::time::Instant,
}

impl Message {
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.user, self.text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    data: (),
    // datetime: std::time::Instant,
}
