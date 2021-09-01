use rusqlite::Connection;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::database;
use crate::networking;
use crate::request::handle_request;
use crate::request::Request;
use crate::Args;

/// setup_server() is a helper function that sets up connection listeners,
/// databases, and creates some channels for replicating the messages.
pub async fn setup_server(args: Arc<Args>) {
    let address = "0.0.0.0:".to_owned() + &args.port;
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let db = Arc::new(Mutex::new(database::open_db("database.db")));

    loop {
        // Wait for clients to connect
        let (conn, _) = listener.accept().await.unwrap();

        // Spawn a new task to handle the client connectison
        spawn(server(conn, Arc::clone(&db)));
    }
}

/// server is a function that handles reading from connections
/// then handles the request and sends the response back to the client.
async fn server(conn: TcpStream, db: Arc<Mutex<Connection>>) {
    let connection: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(conn));

    // Read Connection
    loop {
        // Read bytes from the connection
        let bytes_result = networking::read_from_connection(Arc::clone(&connection)).await;

        if bytes_result.is_some() {
            let bytes = bytes_result.unwrap();

            // Convert bytes to a Request
            let request: Request = bincode::deserialize(&bytes).unwrap();
            println!("Server Received: {:?}", request);

            // Process the response from the request
            let response = handle_request(request, Arc::clone(&db)).await;

            // Sent the response back to the client as bytes
            let bytes = bincode::serialize(&response).unwrap();
            networking::write_to_connection(&bytes, Arc::clone(&connection)).await;
        }
    }
}
