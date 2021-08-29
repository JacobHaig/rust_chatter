use std::sync::Arc;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    database::DataBase::{self, MessageArticals},
    response::{self, Response},
};

/// The client sends a request to the server.
/// The server responds with a response. The
/// response is of type `Response`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    LastMessages(u32),
    AfterTimestamp(u64),
}

pub async fn handle_request(request: Request, db: Arc<Mutex<Connection>>) -> Response {
    let database = db.lock().await;

    match request {
        Request::LastMessages(n) => {
            let articals = DataBase::get_recent_messages(&database, n);

            let messages = articals
                .iter()
                .map(|artical| response::Message {
                    user: artical.username.clone(),
                    text: artical.content.clone() ,
                })
                .collect();

            Response::Message(messages)
        }
        Request::AfterTimestamp(_) => todo!(),
    }
}
