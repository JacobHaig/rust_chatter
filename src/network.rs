use std::io::Read;
use std::io::Write;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

/// Write then Read
pub fn send_get<T, U>(value: T, conn: Arc<Mutex<TcpStream>>) -> U
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
    send(value, conn.clone());
    get(conn).unwrap()
}

/// write_from_connection
pub fn send<T>(value: T, conn: Arc<Mutex<TcpStream>>)
where
    T: serde::Serialize,
{
    let bytes = bincode::serialize(&value).unwrap();

    // get the size of the message
    let byte_size = bincode::serialize(&bytes.len()).unwrap();

    // lock the connection and write the bytes
    if let Ok(mut conn_locked) = conn.lock() {
        // Write the size of the message
        if conn_locked.write_all(&byte_size).is_ok() {
            // then we can write the message
            conn_locked.write_all(&bytes).unwrap();
        }
    }
}

/// read_from_connection reads a message from a connection.
/// It returns an Option<T> where T is the type of the message.
pub fn get<T>(conn: Arc<Mutex<TcpStream>>) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
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

            let request: T = bincode::deserialize(&bytes).unwrap();
            Some(request)
        }
        Err(_) => None,
    }
}
