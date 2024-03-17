use super::DatabaseOperations;
use crate::lib::{Component, Log, Machine};
use chrono::NaiveDateTime;
use rusqlite::{params, Error};

impl DatabaseOperations {
    pub fn get_machine(&self, name: &str) -> Option<Machine> {
        const GET_MACHINE_QUERY: &str = "SELECT id,description FROM Machine WHERE name = ?1";
        match self
            .conn
            .query_row(GET_MACHINE_QUERY, params![name], |row| {
                Ok(Machine {
                    id: row.get(0)?,
                    name: name.to_string(),
                    description: row.get(1)?,
                })
            }) {
            Ok(machine) => Some(machine),
            Err(Error::QueryReturnedNoRows) => None,
            Err(e) => panic!("Error in the database when getting the machine: {}", e),
        }
    }

    /// get a list of components from the database.
    /// # Arguments
    /// 'machine_name' - the name of the machine.
    /// # Returns
    /// A list of components if the machine exists, None otherwise.
    pub fn get_components(&self, machine_name: &str) -> Option<Vec<Component>> {
        // check if the machine exists
        let machine = match self.get_machine(machine_name) {
            Some(machine) => machine,
            None => return None,
        };

        // get the list of components
        const GET_COMPONENTS_QUERY: &str = "SELECT id,name,description,status,priority FROM Component WHERE machineid = ?1 ORDER BY priority";
        let mut stmt = self.conn.prepare(GET_COMPONENTS_QUERY).unwrap();
        let component_iter = stmt
            .query_map(params![machine.id], |row| {
                Ok(Component {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                })
            })
            .unwrap();
        let mut components: Vec<Component> = Vec::new();
        for component in component_iter {
            components.push(component.unwrap());
        }
        Some(components)
    }

    pub fn get_component(&self, component_name: &str) -> Option<Component> {
        const GET_COMPONENT_QUERY: &str =
            "SELECT id,description,status,priority FROM Component WHERE name = ?1";
        match self
            .conn
            .query_row(GET_COMPONENT_QUERY, params![component_name], |row| {
                Ok(Component {
                    id: row.get(0)?,
                    name: component_name.to_string(),
                    description: row.get(1)?,
                    status: row.get(2)?,
                    priority: row.get(3)?,
                })
            }) {
            Ok(component) => Some(component),
            Err(Error::QueryReturnedNoRows) => None,
            Err(e) => panic!("Error in the database when getting the component: {}", e),
        }
    }

    /// get a list of logs from the database.
    /// # Arguments
    /// 'component_name' - the name of the component.
    /// # Returns
    /// A list of logs if the component exists, None otherwise.
    pub fn get_logs(&self, component_name: &str) -> Option<Vec<Log>> {
        // check if the component exists
        let component = match self.get_component(component_name) {
            Some(component) => component,
            None => return None,
        };

        // get the list of logs
        const GET_LOGS_QUERY: &str = "SELECT id,status,name,description,startDate,fixedDate,fixed FROM Log WHERE componentid = ?1";
        let mut stmt = self.conn.prepare(GET_LOGS_QUERY).unwrap();
        let log_iter = stmt
            .query_map(params![component.id], |row| {
                Ok(Log {
                    id: row.get(0)?,
                    status: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    start_date: NaiveDateTime::parse_from_str(
                        row.get::<_, String>(4)?.as_str(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    fixed_date: row.get::<_, Option<String>>(5)?.map(|date: String| {
                        NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap()
                    }),
                    fixed: row.get(6)?,
                })
            })
            .unwrap();

        let mut logs: Vec<Log> = Vec::new();
        for log in log_iter {
            logs.push(log.unwrap());
        }
        Some(logs)
    }

    pub fn get_log(&self, id: i32) -> Option<Log> {
        const GET_LOG_QUERY: &str =
            "SELECT status,name,description,startDate,fixedDate,fixed FROM Log WHERE id = ?1";
        match self.conn.query_row(GET_LOG_QUERY, params![id], |row| {
            Ok(Log {
                id,
                status: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                start_date: NaiveDateTime::parse_from_str(
                    row.get::<_, String>(3)?.as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                fixed: row.get(5)?,
                fixed_date: row.get::<_, Option<String>>(5)?.map(|date: String| {
                    NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap()
                }),
            })
        }) {
            Ok(log) => Some(log),
            Err(Error::QueryReturnedNoRows) => None,
            Err(e) => panic!("Error in the database when getting the log: {}", e),
        }
    }
}
