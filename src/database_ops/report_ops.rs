use super::DatabaseOperations;
use crate::args::ReportGlich;
use rusqlite::{params, Error};

impl DatabaseOperations {
    pub fn handle_report_glich(&self, info: ReportGlich) {
        // check if the component name exists
        const CHECK_COMPONENT_NAME_QUERY: &str = "SELECT id FROM Component WHERE name = ?1";
        let id:i32 = match self
            .conn
            .query_row(CHECK_COMPONENT_NAME_QUERY, params![info.component], |row| {
                row.get::<_, i32>(0)
            }) {
            Ok(id) => id,
            Err(Error::QueryReturnedNoRows) => {
                panic!("Component with the name {} does not exist.", info.component)
            }
            Err(e) => panic!(
                "Error in the database when checking for the component name: {}",
                e
            ),
        };

        // add the glich
        const ADD_GLICH_QUERY: &str = "INSERT INTO Log (componentid,name,description,status) VALUES (?1,?2,?3,COALESCE(?4,2))";
        match self.conn.execute(
            ADD_GLICH_QUERY,
            params![id, info.name, info.description, info.status],
        ) {
            Ok(_) => println!("Glich reported successfully."),
            Err(e) => panic!("Error in the database when adding the glich: {}", e),
        }
    }
}
