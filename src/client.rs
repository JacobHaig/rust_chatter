use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::request::Request;
use crate::response::Response;
use crate::{message, networking, Args};

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
pub async fn client(args: Arc<Args>) {
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
        networking::write_to_connection(&bytes, Arc::clone(&conn)).await;

        /////////////////////////////////////////////////

        // Read Connection
        let bytes_result = networking::read_from_connection(Arc::clone(&conn)).await;

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
