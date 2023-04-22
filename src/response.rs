use serde::{Deserialize, Serialize};

use crate::message::{Message, User};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    OK,
    Error(String),
    Messages(Vec<Message>),
    Users(Vec<User>),
}
