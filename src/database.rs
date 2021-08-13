use crate::database::DataBase::Message;
use rusqlite::Connection;
use time::Date;

pub mod DataBase {

    #[derive(Debug)]
    pub struct Message {
        pub id: i32,
        pub username: String,
        pub content: String,
        pub timestamp: i64,
    }

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
    pub fn delete_all_messages(db: &mut rusqlite::Connection) {
        db.execute("DELETE FROM messages", []).unwrap();
    }

    /// Delete a message from the database.
    pub fn delete_message(db: &mut rusqlite::Connection, id: i32) {
        db.execute("DELETE FROM messages WHERE id = ?", [id])
            .unwrap();
    }

    /// Inserts a new message into the database.
    pub fn insert_message(db: &rusqlite::Connection, message: Message) {
        db.execute(
            "INSERT INTO messages (username, content, timestampms)
            VALUES (?1, ?2, ?3)",
            rusqlite::params![message.username, message.content, message.timestamp],
        )
        .unwrap();
    }

    /// Returns a list of all messages from the database where a condition is met.
    pub fn where_message(db: &rusqlite::Connection, args: &[&str]) -> Vec<Message> {
        let condition = args.join(" AND ");

        
        let mut stmt = db
            .prepare(format!("SELECT * FROM messages WHERE {}", condition).as_str())
            .unwrap();

        // Execute the statement and get the results
        // then convert the results into a vector of messages.
        let query_iter = stmt
            .query_map([], |row| {
                Ok(Message {
                    id: row.get(0).unwrap(),
                    username: row.get(1).unwrap(),
                    content: row.get(2).unwrap(),
                    timestamp: row.get(3).unwrap(),
                })
            })
            .unwrap();

        let result = query_iter
            .into_iter()
            .map(|q| q.unwrap())
            .collect::<Vec<Message>>();

        result
    }
}

/// This start_db function tests the creation of a database,
/// and the insertion of a message, and the retrieval of a message
/// from the database.
#[test]
fn start_db() {
    let timestamp = chrono::Utc::now().timestamp_millis();

    let db: Connection = DataBase::open_db("database.db");

    let message = Message {
        id: 0,
        username: "Andrew".to_string(),
        content: "No".to_string(),
        timestamp: timestamp,
    };

    DataBase::insert_message(&db, message);

    let results = DataBase::where_message(&db, &["timestampms > 1627931795666"]);

    for message in results {
        println!("Found person {:?}", message);
    }
}
