use crate::message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    OK,
    Error(String),
    Message(Vec<message::Message>),
    // Image(Image),
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Image {
//     data: (),
//     // datetime: std::time::Instant,
// }
