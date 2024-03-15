use crate::Connection;
mod add_ops;

struct database_operations {
    conn: Connection,
}

impl database_operations {
    fn new(database_path: &str) -> database_operations {
        let conn = Connection::open(database_path).unwrap();
        conn.execute_batch(include_str!("../sql/setup.sql"))
            .expect("Error setting up the database");
        database_operations { conn }
    }
}
