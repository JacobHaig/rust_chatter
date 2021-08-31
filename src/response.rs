use crate::message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    OK,
    Error(String),
    Message(Vec<message::Message>),
    // Image(Image),
}

