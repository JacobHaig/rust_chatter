use clap::Clap;
use message::Message;
use std::env::args;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
// std::sync::Mutex is not Send and will cause problems
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::message::Packet;

mod message;

// Command line arguments datastructure. The inputs and parameters
// are handled at compile time by CLAP.
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

#[tokio::main]
async fn main() {
    let args: &Args = &Args::parse();
    println!("{:?}", args);

    // We only run a server or client. Client by default.
    if args.is_server {
        setup_server(args).await
    } else {
        client(args).await
    }
}

async fn setup_server(args: &Args) {
    let address = "0.0.0.0:".to_owned() + &args.port;
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let atx = Arc::new(tx);

    // Write Connection
    spawn(replicate(Arc::clone(&connections), rx));

    loop {
        // Wait for clients to connect
        let (conn, _) = listener.accept().await.unwrap();

        // Push new connections to the connections vec
        let mut lock_connections = connections.lock().await;
        let new_conn = Arc::new(Mutex::new(conn));
        lock_connections.push(Arc::clone(&new_conn));

        spawn(server(new_conn, Arc::clone(&atx)));
    }
}

async fn replicate(connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>, mut rx: Receiver<Vec<u8>>) {
    loop {
        // Wait for bytes to be sent over the channel.
        // The bytes comes from client streams
        let bytes = rx.recv().await.unwrap();

        // Write bytes to each stream
        for conn in connections.lock().await.iter() {
            write_to_connection(&bytes, Arc::clone(conn)).await;
        }
    }
}

async fn server(conn: Arc<Mutex<TcpStream>>, atx: Arc<Sender<Vec<u8>>>) {
    // Read Connection
    loop {
        // Read bytes so we can send over channel
        let bytes_result = read_from_connection(conn.clone()).await;

        if bytes_result.is_some() {
            let bytes = bytes_result.unwrap();

            let packet: Packet = bincode::deserialize(&bytes).unwrap();
            println!("Server Received: {:?}", packet);

            // Move bytes into channel
            atx.send(bytes).await.unwrap();
        }
    }
}

async fn client(args: &Args) {
    let address = format!("{}:{}", args.address, args.port);
    let conn = tokio::net::TcpStream::connect(address).await.unwrap();
    let aconn = Arc::new(Mutex::new(conn));

    // Write Connection
    spawn(write_client(Arc::clone(&aconn)));

    // Read Connection
    loop {
        let bytes_result = read_from_connection(Arc::clone(&aconn)).await;

        if bytes_result.is_some() {
            let bytes = bytes_result.unwrap();

            let packet: Packet = bincode::deserialize(&bytes).unwrap();

            if let Packet::Message(m) = packet {
                println!("{}", m.to_string());
            }
        }
    }
}

async fn write_client(aconn: Arc<Mutex<TcpStream>>) {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();
        let input_cleaned = input[0..input.len() - 2].to_string();

        let packet = Packet::Message(Message {
            user: "USERNAME".to_string(),
            text: input_cleaned,
        });

        let bytes = bincode::serialize(&packet).unwrap();
        write_to_connection(&bytes, Arc::clone(&aconn)).await;
    }
}

async fn write_to_connection(bytes: &[u8], conn: Arc<Mutex<TcpStream>>) {
    let byte_size = bincode::serialize(&bytes.len()).unwrap();

    let mut conn_locked = conn.lock().await;

    match conn_locked.try_write(&byte_size) {
        Ok(_) => conn_locked.write_all(bytes).await.unwrap(),
        Err(_) => {}
    }
}

async fn read_from_connection(conn: Arc<Mutex<TcpStream>>) -> Option<Vec<u8>> {
    let mut byte_size = [0u8; 8];
    let mut conn_locked = conn.lock().await;

    match conn_locked.try_read(&mut byte_size) {
        Ok(_) => {
            let size: u64 = bincode::deserialize(&byte_size).expect("Deserialize size failed");

            let mut bytes = vec![0; size as usize];
            conn_locked
                .read_exact(&mut bytes)
                .await
                .expect("Cant read byte");
            Some(bytes)
        }
        Err(_) => None,
    }
}
