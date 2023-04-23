use std::io::Read;
use std::io::Write;

use std::net::TcpStream;
use std::sync::Arc;

/// Write then Read
pub fn send_get<T, U>(value: T, conn: Arc<TcpStream>) -> U
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
    send(value, conn.clone());
    get(conn).unwrap()
}

/// write_from_connection
pub fn send<T>(value: T, conn: Arc<TcpStream>)
where
    T: serde::Serialize,
{
    let bytes = bincode::serialize(&value).unwrap();

    // Get the size of the message
    let byte_size = bincode::serialize(&bytes.len()).unwrap();

    if let Ok(mut conn_locked) = conn.try_clone() {
        // Send the size of the message
        if conn_locked.write_all(&byte_size).is_ok() {
            // Then send the message
            conn_locked.write_all(&bytes).unwrap();
        }
    }
}

/// read_from_connection reads a message from a connection.
/// It returns an Option<T> where T is the type of the message.
pub fn get<T>(conn: Arc<TcpStream>) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut byte_size = [0u8; 8];
    let mut conn_locked = conn.try_clone().unwrap();

    // Read the size of the message.
    if conn_locked.read_exact(&mut byte_size).is_ok() {
        let size: u64 = bincode::deserialize(&byte_size).expect("Deserialize size failed");

        // Using the size, allocate buffer of that size
        let mut buffer = vec![0; size as usize];

        // Then read the message into the buffer
        conn_locked
            .read_exact(&mut buffer)
            .expect("Reading Bytes into buffer failed");

        let request: T = bincode::deserialize(&buffer).unwrap();
        return Some(request);
    }
    None
}
