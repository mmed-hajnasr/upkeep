use super::DatabaseOperations;
use crate::args::ReportGlich;
use rusqlite::params;

impl DatabaseOperations {
    pub fn handle_report_glich(&self, info: ReportGlich) {
        // check if the machine exists
        let component = match self.get_component(&info.component) {
            Some(component) => component,
            None => {
                println!("there is no component with the name {}", info.component);
                return;
            }
        };

        // add the glich
        const ADD_GLICH_QUERY: &str = "INSERT INTO Log (componentid,name,description,status) VALUES (?1,?2,?3,COALESCE(?4,2))";
        match self.conn.execute(
            ADD_GLICH_QUERY,
            params![component.id, info.name, info.description, info.status],
        ) {
            Ok(_) => println!("Glich reported successfully."),
            Err(e) => panic!("Error in the database when adding the glich: {}", e),
        }
    }
}
