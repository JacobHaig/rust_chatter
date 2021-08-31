use std::sync::Arc;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{database, message, response::Response};

/// The client sends a request to the server.
/// The server responds with a response. The
/// response is of type `Response`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    LastMessages(u32),
    AfterTimestamp(i64),
    AddMessage(message::Message),
    // NewMessages(u64),
}

pub async fn handle_request(request: Request, db: Arc<Mutex<Connection>>) -> Response {
    let database = db.lock().await;

    match request {
        Request::LastMessages(n) => {
            let articals = database::get_recent_messages(&database, n);

            let messages = articals
                .iter()
                .map(|artical| message::Message {
                    user: artical.username.clone(),
                    text: artical.content.clone(),
                })
                .collect();

            Response::Message(messages)
        }
        Request::AfterTimestamp(n) => {
            let condishion = &("timestampms > ".to_string() + n.to_string().as_str());

            let articals = database::where_message(&database, &[condishion]);

            let messages = articals
                .iter()
                .map(|artical| message::Message {
                    user: artical.username.clone(),
                    text: artical.content.clone(),
                })
                .collect();

            Response::Message(messages)
        }
        Request::AddMessage(message) => {
            database::add_message(&database, message);
            Response::OK
        }
    }
}
