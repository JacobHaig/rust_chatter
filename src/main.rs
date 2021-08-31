use clap::Clap;
use rusqlite::Connection;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::request::handle_request;
use crate::request::Request;
use crate::response::Response;

mod database;
mod message;
mod request;
mod response;

/// Args is a struct that contains the command line arguments.
/// The purpose of this struct is to make it easy to add new
/// command line arguments using Clap.
#[derive(Debug, Clap)]
#[clap(name = "Rust Chatter")]
struct Args {
    #[clap(short = 's', long = "server")]
    is_server: bool,

    #[clap(short, long, required = false, default_value = "127.0.0.1")]
    address: String,

    #[clap(short, long, required = false, default_value = "23432")]
    port: String,

    #[clap(short, long, required = false, default_value = "unknown")]
    username: String,
}

/// The main function is the entry point for the program.
#[tokio::main]
async fn main() {
    let args: Arc<Args> = Arc::new(Args::parse());
    println!("{:?}", args);

    // We only run a server or client. Client by default.
    if args.is_server {
        setup_server(args).await
    } else {
        client(args).await
    }
}

/// setup_server() is a helper function that sets up connection listeners,
/// databases, and creates some channels for replicating the messages.
async fn setup_server(args: Arc<Args>) {
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
        let bytes_result = read_from_connection(Arc::clone(&connection)).await;

        if bytes_result.is_some() {
            let bytes = bytes_result.unwrap();

            // Convert bytes to a Request
            let request: Request = bincode::deserialize(&bytes).unwrap();
            println!("Server Received: {:?}", request);

            // Process the response from the request
            let response = handle_request(request, Arc::clone(&db)).await;

            // Sent the response back to the client as bytes
            let bytes = bincode::serialize(&response).unwrap();
            write_to_connection(&bytes, Arc::clone(&connection)).await;
        }
    }
}

/// client_input gets input from the user and writes it to the stream.
async fn client_input(tx: Sender<Request>, args: Arc<Args>) {
    loop {
        let mut input = String::new();
        // Read input from stdin.
        std::io::stdin().read_line(&mut input).unwrap();
        let input_cleaned = input[0..input.len() - 2].to_string();

        let message = message::Message::new(args.username.clone(), input_cleaned);
        let request = Request::AddMessage(message);
        // let response = Response::Message(vec![message]);

        tx.send(request).unwrap();
    }
}

/// client is a function that handles the connection and
/// reads the messages from the stream then print the message
/// to the screen.
async fn client(args: Arc<Args>) {
    let address = format!("{}:{}", args.address, args.port);

    // Connect to the server and get a stream
    let conn = match tokio::net::TcpStream::connect(address).await {
        Ok(conn) => conn,
        Err(_) => {
            println!("Could not connect to server. ");
            // Quit the program -- This is not a graceful way to quit.
            std::process::exit(0);
        }
    };
    let conn = Arc::new(Mutex::new(conn));

    // Create a channel for reading user input
    let (tx, rx) = std::sync::mpsc::channel();
    spawn(client_input(tx, Arc::clone(&args)));

    let timestamp = chrono::Utc::now().timestamp_millis();

    // This is the main loop for the client where it creates a Request
    // and sends it to the server. Then it waits for the Response and
    // prints it to the screen.
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f32(1.0)).await;

        // Write Connection
        // Get request from the channel or request the last message
        let request = rx.try_recv().unwrap_or(Request::AfterTimestamp(timestamp));

        let bytes = bincode::serialize(&request).unwrap();
        write_to_connection(&bytes, Arc::clone(&conn)).await;

        /////////////////////////////////////////////////

        // Read Connection
        let bytes_result = read_from_connection(Arc::clone(&conn)).await;

        if bytes_result.is_some() {
            let bytes = bytes_result.unwrap();

            let response: Response = bincode::deserialize(&bytes).unwrap();

            // if packet is a message, print it to the screen
            if let Response::Message(message) = response {
                for m in message {
                    println!("{}", m.to_string());
                }
            }
        }
    }
}

/// write_from_connection() takes a connection and writes the bytes
/// to the TCP stream. We write the message in to steps.
/// First we write the size of the message. Then Second we write the message
/// itself.
async fn write_to_connection(bytes: &[u8], conn: Arc<Mutex<TcpStream>>) {
    // get the size of the bytes in bytes
    let byte_size = bincode::serialize(&bytes.len()).unwrap();

    let mut conn_locked = conn.lock().await;

    // Write the size of the message
    if conn_locked.try_write(&byte_size).is_ok() {
        // then we can write the message
        conn_locked.write_all(bytes).await.unwrap()
    }
}

/// read_from_connection() takes a connection and reads the bytes
/// from the TCP stream. We read the message in two steps.
/// First we read the size of the message. Then Second we read the message
/// using the size we read in the first step and return an optional vec of bytes.
async fn read_from_connection(conn: Arc<Mutex<TcpStream>>) -> Option<Vec<u8>> {
    let mut byte_size = [0u8; 8];
    let mut conn_locked = conn.lock().await;

    // Read the size of the message and deserialize it.
    match conn_locked.try_read(&mut byte_size) {
        Ok(_) => {
            let size: u64 = bincode::deserialize(&byte_size).expect("Deserialize size failed");

            // once we have the size we can allocate a vec of bytes
            let mut bytes = vec![0; size as usize];
            // and read the message into the vec
            conn_locked
                .read_exact(&mut bytes)
                .await
                .expect("Cant read byte");
            Some(bytes)
        }
        Err(_) => None,
    }
}
