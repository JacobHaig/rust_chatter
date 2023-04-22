use turbosql::Turbosql;

#[derive(Turbosql, Default, Debug, Clone)]
pub struct MessageRow {
    pub rowid: Option<i64>,
    pub username: Option<String>,
    pub content: Option<String>,
    pub timestamp_ms: Option<i64>,
}

#[derive(Turbosql, Default, Debug, Clone)]
pub struct UserRow {
    pub rowid: Option<i64>,
    pub username: Option<String>,
}

pub fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// Message Namespace
pub mod message {
    use super::MessageRow;
    use crate::message::Message;
    use turbosql::{execute, select, Turbosql};

    /// Creates a new message
    #[allow(dead_code)]
    pub fn add(username: String, content: String) {
        MessageRow {
            username: Some(username),
            content: Some(content),
            timestamp_ms: Some(super::current_timestamp()),
            ..Default::default()
        }
        .insert()
        .unwrap();
    }

    // Add a message to the database
    pub fn add_message(message: Message) {
        let mut message = message.to_row();

        // Override the timestamp so that it matches the server's time
        message.timestamp_ms = Some(super::current_timestamp());

        message.insert().unwrap();
    }

    // From MessageRow to Message
    impl From<MessageRow> for Message {
        fn from(row: MessageRow) -> Self {
            Message {
                id: row.rowid.unwrap() as i32,
                username: row.username.unwrap(),
                content: row.content.unwrap(),
                timestamp_ms: row.timestamp_ms.unwrap(),
            }
        }
    }

    pub fn to_messages(rows: Vec<MessageRow>) -> Vec<Message> {
        rows.into_iter().map(|row| row.into()).collect()
    }

    // Select all messages
    #[allow(dead_code)]
    pub fn select_all() -> Vec<MessageRow> {
        select!(Vec<MessageRow>).unwrap()
    }

    // Select a message after timestamp
    pub fn select_after(timestamp_ms: u64) -> Vec<MessageRow> {
        select!(Vec<MessageRow> "WHERE timestamp_ms >" timestamp_ms).unwrap()
    }

    // Select most recent message
    pub fn select_last(i: u32) -> Vec<MessageRow> {
        select!(Vec<MessageRow> "ORDER BY timestamp_ms DESC LIMIT" i).unwrap()
    }

    // Delete all messages
    #[allow(dead_code)]
    pub fn delete_all() {
        execute!("DELETE FROM MESSAGEROW").unwrap();
    }
}

// User Namespace
pub mod user {
    use super::UserRow;
    use crate::message::User;
    use turbosql::{execute, select, Turbosql};

    pub fn add_user(user: User) {
        UserRow {
            username: Some(user.username),
            ..Default::default()
        }
        .insert()
        .unwrap();
    }

    pub fn select_all() -> Vec<UserRow> {
        select!(Vec<UserRow>).unwrap()
    }

    // From UserRow to User
    impl From<UserRow> for User {
        fn from(row: UserRow) -> Self {
            User {
                id: row.rowid.unwrap() as i32,
                username: row.username.unwrap(),
            }
        }
    }

    pub fn to_users(rows: Vec<UserRow>) -> Vec<User> {
        rows.into_iter().map(|row| row.into()).collect()
    }

    // Delete all users
    #[allow(dead_code)]
    pub fn delete_all() {
        execute!("DELETE FROM USERROW").unwrap();
    }

    pub fn remove(username: String) {
        execute!("DELETE FROM USERROW WHERE username = ?", username).unwrap();
    }
}

//
// Test Cases
#[test]
fn test_add_message_row() {
    MessageRow {
        username: Some("bob".to_string()),
        content: Some("CONTENTER!".to_string()),
        ..Default::default()
    }
    .insert()
    .unwrap();
}

#[test]
fn test_select_all() {
    let messages = message::select_all();

    dbg!(&messages);

    assert_eq!(messages.len(), 1);
}

// Test adding a message
#[test]
fn test_add_message() {
    let message = crate::message::Message::new("bob".to_string(), "CONTENT!".to_string());
    message::add_message(message);
}

// Delete all messages
#[test]
fn test_delete_all() {
    message::delete_all();
    user::delete_all();
}
