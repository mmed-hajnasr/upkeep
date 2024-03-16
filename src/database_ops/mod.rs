use rusqlite::Connection;
mod add_ops;
mod show_ops;
mod report_ops;

pub struct DatabaseOperations {
    conn: Connection,
}

impl DatabaseOperations {
    pub fn new(database_path: &str) -> DatabaseOperations {
        let conn = Connection::open(database_path).unwrap();
        conn.execute_batch(include_str!("../sql/setup.sql"))
            .expect("Error setting up the database");
        DatabaseOperations { conn }
    }
}