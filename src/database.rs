use crate::database::DataBase::Message;
use rusqlite::Connection;

mod DataBase {

    #[derive(Debug)]
    pub struct Message {
        pub id: i32,
        pub username: String,
        pub content: String,
        pub timestamp: i64,
    }

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

    pub fn insert_message(db: &rusqlite::Connection, message: Message) {
        db.execute(
            "INSERT INTO messages (username, content, timestampms)
            VALUES (?1, ?2, ?3)",
            rusqlite::params![message.username, message.content, message.timestamp],
        )
        .unwrap();
    }

    // pub fn remove(&Connection, message: Message) {
    //     todo!()
    // }

    pub fn where_message(db: &rusqlite::Connection, args: &[&str]) -> Vec<Message> {
        let condition = args.join(" AND ");

        let mut stmt = db
            .prepare(format!("SELECT * FROM messages WHERE {}", condition).as_str())
            .unwrap();

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
