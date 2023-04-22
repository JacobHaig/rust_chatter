// use rusqlite::Connection;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use crate::network;
use crate::request::handle_request;
use crate::Args;

/// setup_server() is a helper function that sets up connection listeners,
/// and spawns a new thread for each connection.
pub fn setup_server(args: Arc<Args>) {
    let address = "0.0.0.0:".to_owned() + &args.port;
    let listener = std::net::TcpListener::bind(address).unwrap();

    loop {
        let (conn, _) = listener.accept().unwrap();
        std::thread::spawn(|| server(conn));
    }
}

/// server() is the main function for the server.
fn server(conn: TcpStream) {
    let connection: &Arc<Mutex<TcpStream>> = &Arc::new(Mutex::new(conn));

    // Read, Handle, Write, Loop
    loop {
        let request_result = network::get(connection.clone());

        if let Some(request) = request_result {
            let response = handle_request(request);

            network::send(response, connection.clone());
        }
    }
}
