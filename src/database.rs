use rusqlite::Connection;

use crate::message;

/// Create a new database connection if database doesn't exist.
pub fn open_db(db_name: &str) -> rusqlite::Connection {
    let db = rusqlite::Connection::open(db_name).unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS messages (
                id              INTEGER PRIMARY KEY,
                username        TEXT    NOT NULL, 
                content         TEXT    NOT NULL, 
                timestampms     INTEGER NOT NULL
            )",
        [],
    )
    .unwrap();

    db.execute(
        "create table if not exists users (
                username    TEXT NOT NULL, 
                uuid        TEXT NOT NULL
            )",
        [],
    )
    .unwrap();

    db
}

/// Deletes all messages from the database.
#[allow(dead_code)]
pub fn delete_all_messages(db: &mut rusqlite::Connection) {
    db.execute("DELETE FROM messages", []).unwrap();
}

/// Delete a message from the database.
#[allow(dead_code)]
pub fn delete_message(db: &mut rusqlite::Connection, id: i32) {
    db.execute("DELETE FROM messages WHERE id = ?", [id])
        .unwrap();
}

/// Adds a new message into the database.
pub fn add_message(db: &rusqlite::Connection, message: message::Message) {
    db.execute(
        "INSERT INTO messages (username, content, timestampms)
        VALUES (?1, ?2, ?3)",
        rusqlite::params![
            message.username,
            message.content,
            chrono::Utc::now().timestamp_millis()
        ],
    )
    .unwrap();
}

/// Returns a list of all messages from the database where a condition is met.
pub fn where_message(db: &rusqlite::Connection, args: &[&str]) -> Vec<message::Message> {
    let condition = args.join(" AND ");
    let query = format!("SELECT * FROM messages WHERE {}", condition);

    get_messages(db, query)
}

/// Returns the most recent messages from the database.
pub fn get_recent_messages(db: &rusqlite::Connection, amount: u32) -> Vec<message::Message> {
    let query = format!(
        "SELECT * FROM messages ORDER BY timestampms DESC LIMIT {}",
        amount
    );

    get_messages(db, query)
}

/// Returns the most recent messages from the database.
#[allow(dead_code)]
pub fn get_messages_at_index(db: &rusqlite::Connection, index: u32) -> Vec<message::Message> {
    let query = format!("SELECT * FROM messages WHERE id = {}", index);

    get_messages(db, query)
}

fn get_messages(db: &Connection, query: String) -> Vec<message::Message> {
    let mut stmt = db.prepare(&query).unwrap();

    let query_iter = stmt
        .query_map([], |row| {
            Ok(message::Message {
                id: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                content: row.get(2).unwrap(),
                timestamp: row.get(3).unwrap(),
            })
        })
        .unwrap();
    query_iter
        .into_iter()
        .map(|q| q.unwrap())
        .collect::<Vec<message::Message>>()
}

/// This start_db function tests the creation of a database,
/// and the insertion of a message, and the retrieval of a message
/// from the database.
#[test]
fn start_db() {
    let timestamp = chrono::Utc::now().timestamp_millis();

    let db: Connection = open_db("database.db");

    let message = message::Message {
        id: 0,
        username: "Andrew".to_string(),
        content: "No".to_string(),
        timestamp: timestamp,
    };

    // insert_message(&db, message);

    // This is a test to see if the message is inserted into the database.
    let results = where_message(&db, &["timestampms > 1627931795666"]);

    for message in results {
        println!("1: Found person {:?}", message);
    }

    // This is a test to see if we an abritrary amount of messages.
    let results = get_recent_messages(&db, 1);

    for message in results {
        println!("2: Found person {:?}", message);
    }
}
