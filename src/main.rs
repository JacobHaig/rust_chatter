use clap::Clap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
// std::sync::Mutex is not Send and will cause problems
use tokio::sync::Mutex;
use tokio::task::spawn;

// static mut connections: Vec<&TcpSocket> = Vec::new();

#[derive(Debug, Clap)]
#[clap(name = "Rust Chatter")]
struct Args {
    #[clap(short = 's', long = "server")]
    is_server: bool,

    #[clap(short, long, required = false, default_value = "127.0.0.1")]
    address: String,

    #[clap(short, long, required = false, default_value = "23432")]
    port: String,
}

#[tokio::main]
async fn main() {
    let args: &Args = &Args::parse();

    println!("{:?}", args);

    if args.is_server {
        setup_server(args).await
    } else {
        client(args).await
    }
}

async fn setup_server(args: &Args) {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + &args.port)
        .await
        .unwrap();

    let connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let atx = Arc::new(tx);

    // Write Connection
    spawn(replicate(Arc::clone(&connections), rx));

    loop {
        let (conn, _addr) = listener.accept().await.unwrap();

        // let mut lock = connections.clone().lock().unwrap();
        let mut lock_connections = connections.lock().await;
        let new_conn = Arc::new(Mutex::new(conn));
        lock_connections.push(Arc::clone(&new_conn));

        spawn(server(new_conn, Arc::clone(&atx)));
    }
}

async fn replicate(connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>, mut rx: Receiver<Vec<u8>>) {
    loop {
        let data = rx.recv().await.unwrap();

        let lock_connections = connections.lock().await;

        for conn in lock_connections.iter() {
            write_to_connection(&data, Arc::clone(conn)).await;
        }
    }
}

async fn server(conn: Arc<Mutex<TcpStream>>, tx: Arc<Sender<Vec<u8>>>) {
    // Read Connection
    loop {
        let data = read_from_connection(conn.clone()).await;

        if data.is_some() {
            let data = data.unwrap();

            let content: String = bincode::deserialize(&data).unwrap();
            println!("Server Received: {:?}", content);

            tx.send(data).await.unwrap();
        }
    }
}

async fn client(args: &Args) {
    let address = format!("{}:{}", args.address, args.port);
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let astream = Arc::new(Mutex::new(stream));

    // Write Connection
    spawn(write_client(Arc::clone(&astream)));

    // Read Connection
    loop {
        let data = read_from_connection(Arc::clone(&astream)).await;

        if data.is_some() {
            let data = data.unwrap();

            let content: String = bincode::deserialize(&data).unwrap();

            println!("Client Received: {}", content);
        }
    }
}

async fn write_client(astream: Arc<Mutex<TcpStream>>) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input_cleaned = input[0..input.len() - 2].to_string();

        let data = bincode::serialize(&input_cleaned).unwrap();

        write_to_connection(&data, Arc::clone(&astream)).await;
    }
}

async fn write_to_connection(data: &[u8], conn: Arc<Mutex<TcpStream>>) {
    let size = bincode::serialize(&data.len()).unwrap();

    let mut stream = conn.lock().await;

    stream
        .write_all(&size)
        .await
        .expect("Could not write size bytes to stream. ");

    stream
        .write_all(data)
        .await
        .expect("Could not write bytes to stream. ");
}

async fn read_from_connection(conn: Arc<Mutex<TcpStream>>) -> Option<Vec<u8>> {
    let mut data_size = [0u8; 8];

    let mut stream = conn.lock().await;

    match stream.try_read(&mut data_size) {
        Ok(_) => {
            let size: u64 =
                bincode::deserialize(&data_size).expect("Could not deserialize size correctly");

            let mut data = vec![0; size as usize];
            stream
                .read_exact(&mut data)
                .await
                .expect("Can not Read data from stream ");
            Some(data)
        }
        Err(_) => None,
    }
}
