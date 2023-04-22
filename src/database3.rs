use rusqlite::Connection;

use crate::message;

pub fn open_db(db_name: &str) -> rusqlite::Connection {
    let database = rusqlite::Connection::open(db_name).unwrap();

    database.execute(
        "CREATE TABLE IF NOT EXISTS MESSAGES (
            ID              INTEGER PRIMARY KEY,
            USERNAME        TEXT    NOT NULL, 
            MESSAGE         TEXT    NOT NULL, 
            TIMESTAMP_MS    INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();

    database.execute(
        "CREATE TABLE IF NOT EXISTS USERS (
            USERNAME    TEXT NOT NULL, 
            UUID        TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    database
}

/// Deletes all messages from the database.
#[allow(dead_code)]
pub fn delete_all_messages(database: &mut rusqlite::Connection) {
    database.execute("DELETE FROM MESSAGES", []).unwrap();
}

/// Delete a message from the database.
#[allow(dead_code)]
pub fn delete_message(database: &mut rusqlite::Connection, id: i32) {
    database
        .execute("DELETE FROM MESSAGES WHERE ID = ?", [id])
        .unwrap();
}

/// Adds a new message into the database.
pub fn add_message(database: &rusqlite::Connection, message: message::Message) {
    let now = chrono::Utc::now();

    database
        .execute(
            "INSERT INTO MESSAGES 
            (USERNAME, CONTENT, TIMESTAMP_MS)
            VALUES (?1, ?2, ?3)",
            rusqlite::params![message.username, message.content, now.timestamp_millis()],
        )
        .unwrap();
}

/// Returns a list of all messages from the database where a condition is met.
pub fn where_message(database: &rusqlite::Connection, args: &[&str]) -> Vec<message::Message> {
    let condition = args.join(" AND ");
    let query = format!(
        "SELECT * 
        FROM MESSAGES
        WHERE {condition}"
    );

    get_messages(database, query)
}

/// Returns the most recent messages from the database.
pub fn get_recent_messages(database: &rusqlite::Connection, amount: u32) -> Vec<message::Message> {
    let query = format!(
        "SELECT * FROM MESSAGES 
        ORDER BY TIMESTAMP_MS 
        DESC LIMIT {amount}"
    );

    get_messages(database, query)
}

/// Returns the most recent messages from the database.
#[allow(dead_code)]
pub fn get_messages_at_index(database: &rusqlite::Connection, index: u32) -> Vec<message::Message> {
    let query = format!(
        "SELECT * FROM MESSAGES
        WHERE ID = {index}"
    );

    get_messages(database, query)
}

fn get_messages(database: &Connection, query: String) -> Vec<message::Message> {
    let mut stmt = database.prepare(&query).unwrap();

    stmt
        .query_map([], |row| {
            Ok(message::Message {
                id: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                content: row.get(2).unwrap(),
                timestamp_ms: row.get(3).unwrap(),
            })
        })
        .unwrap()
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

    let _message = message::Message {
        id: 0,
        username: "Andrew".to_string(),
        content: "No".to_string(),
        timestamp_ms: timestamp,
    };

    // insert_message(&db, message);

    // This is a test to see if the message is inserted into the database.
    let results = where_message(&db, &["TIMESTAMP_MS > 1627931795666"]);

    for message in results {
        println!("1: Found person {:?}", message);
    }

    // This is a test to see if we an abritrary amount of messages.
    let results = get_recent_messages(&db, 1);

    for message in results {
        println!("2: Found person {:?}", message);
    }
}
