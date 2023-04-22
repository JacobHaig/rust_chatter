use serde::{Deserialize, Serialize};

use crate::{
    database::{self, message::to_messages, user::to_users},
    message::{Message, User},
    response::Response,
};

/// The client sends a request to the server.
/// The server responds with a response. The
/// response is of type `Response`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    AddMessage(Message),
    GetMessages(),
    LastMessages(u32),
    AfterTimestamp(u64),
    GetMessageAtIndex(u32),
    GetUsers(),
    AddUser(User),
    RemoveUser(User),
}

pub fn handle_request(request: Request) -> Response {
    match request {
        Request::AddMessage(message) => {
            database::message::add_message(message);
            Response::OK
        }
        Request::LastMessages(n) => {
            let messages = to_messages(database::message::select_last(n));
            Response::Messages(messages)
        }
        Request::GetMessages() => {
            let messages = to_messages(database::message::select_all());
            Response::Messages(messages)
        }
        Request::AfterTimestamp(n) => {
            let messages = to_messages(database::message::select_after(n));
            Response::Messages(messages)
        }
        Request::GetMessageAtIndex(_) => todo!(),

        Request::AddUser(user) => {
            database::user::add_user(user);
            Response::OK
        }
        Request::GetUsers() => {
            let user = to_users(database::user::select_all());
            Response::Users(user)
        }
        Request::RemoveUser(user) => {
            database::user::remove(user.username);
            Response::OK
        }
    }
}
