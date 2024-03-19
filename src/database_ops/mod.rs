use std::path::PathBuf;

use rusqlite::Connection;
mod add_ops;
mod show_ops;
mod report_ops;
mod output_ops;
mod fix_ops;
mod edit_ops;
mod remove_ops;

pub struct DatabaseOperations {
    conn: Connection,
}

impl DatabaseOperations {
    pub fn new(database_path: PathBuf) -> DatabaseOperations {
        let conn = Connection::open(database_path).unwrap();
        conn.execute_batch(include_str!("../sql/setup.sql"))
            .expect("Error setting up the database");
        DatabaseOperations { conn }
    }
}