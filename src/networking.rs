use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// write_from_connection() takes a connection and writes the bytes
/// to the TCP stream. We write the message in to steps.
/// First we write the size of the message. Then Second we write the message
/// itself.
pub async fn write_to_connection_async(bytes: &[u8], conn: Arc<Mutex<TcpStream>>) {
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
pub async fn read_from_connection_async(conn: Arc<Mutex<TcpStream>>) -> Option<Vec<u8>> {
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

/// write_from_connection() takes a connection and writes the bytes
/// to the TCP stream. We write the message in to steps.
/// First we write the size of the message. Then Second we write the message
/// itself.
pub fn write_to_connection(bytes: &[u8], conn: Arc<std::sync::Mutex<std::net::TcpStream>>) {
    // get the size of the bytes in bytes
    let byte_size = bincode::serialize(&bytes.len()).unwrap();

    let mut conn_locked = conn.lock().unwrap();

    // Write the size of the message
    if conn_locked.write_all(&byte_size).is_ok() {
        // then we can write the message
        conn_locked.write_all(bytes).unwrap();
    }
}

/// read_from_connection() takes a connection and reads the bytes
/// from the TCP stream. We read the message in two steps.
/// First we read the size of the message. Then Second we read the message
/// using the size we read in the first step and return an optional vec of bytes.
pub fn read_from_connection(conn: Arc<std::sync::Mutex<std::net::TcpStream>>) -> Option<Vec<u8>> {
    let mut byte_size = [0u8; 8];
    let mut conn_locked = conn.lock().unwrap();

    // Read the size of the message and deserialize it.
    match conn_locked.read_exact(&mut byte_size) {
        Ok(_) => {
            let size: u64 = bincode::deserialize(&byte_size).expect("Deserialize size failed");

            // once we have the size we can allocate a vec of bytes
            let mut bytes = vec![0; size as usize];
            // and read the message into the vec
            conn_locked.read_exact(&mut bytes).expect("Cant read byte");
            Some(bytes)
        }
        Err(_) => None,
    }
}
