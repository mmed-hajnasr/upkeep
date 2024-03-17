use super::DatabaseOperations;
use crate::args::{AddComponent, AddEntityCommand, AddMachine};
use rusqlite::{params, Error};

impl DatabaseOperations {
    pub fn handle_add_machine(&self, info: AddMachine) {
        // check if the machine name already exists
        const CHECK_MACHINE_NAME_QUERY: &str = "SELECT name FROM Machine WHERE name = ?1";
        match self
            .conn
            .query_row(CHECK_MACHINE_NAME_QUERY, params![info.name], |row| {
                row.get::<_, String>(0)
            }) {
            Ok(_) => panic!("Machine with the name {} already exists.", info.name),
            Err(Error::QueryReturnedNoRows) => (),
            Err(e) => panic!(
                "Error in the database when checking for the machine name: {}",
                e
            ),
        }

        // add the machine
        const ADD_MACHINE_QUERY: &str = "INSERT INTO Machine (name, description) VALUES (?1, ?2)";
        match self
            .conn
            .execute(ADD_MACHINE_QUERY, params![info.name, info.description])
        {
            Ok(_) => println!("Machine added successfully."),
            Err(e) => println!("Error in the database when adding the machine: {}", e),
        }
    }

    pub fn handle_add_component(&self, info: AddComponent) {
        // check if the component name already exists
        const CHECK_COMPONENT_NAME_QUERY: &str = "SELECT name FROM Component WHERE name = ?1";
        match self
            .conn
            .query_row(CHECK_COMPONENT_NAME_QUERY, params![info.name], |row| {
                row.get::<_, String>(0)
            }) {
            Ok(_) => panic!("Component with the name {} already exists.", info.name),
            Err(Error::QueryReturnedNoRows) => (),
            Err(e) => panic!(
                "Error in the database when checking for the component name: {}",
                e
            ),
        }

        // check if the machine name exists
        const CHECK_MACHINE_NAME_QUERY: &str = "SELECT name FROM Machine WHERE name = ?1";
        match self
            .conn
            .query_row(CHECK_MACHINE_NAME_QUERY, params![info.machine], |row| {
                row.get::<_, String>(0)
            }) {
            Ok(_) => (),
            Err(Error::QueryReturnedNoRows) => {
                panic!("Machine with the name {} does not exist.", info.machine)
            }
            Err(e) => panic!(
                "Error in the database when checking for the machine name: {}",
                e
            ),
        }

        // add the component
        const ADD_COMPONENT_QUERY: &str = "INSERT INTO Component (name, description, machineid, priority) VALUES (?1, ?2, (SELECT id FROM Machine WHERE name = ?3), COALESCE(?4, 3))";
        match self.conn.execute(
            ADD_COMPONENT_QUERY,
            params![info.name, info.description, info.machine, info.priority],
        ) {
            Ok(_) => println!("Component added successfully."),
            Err(e) => println!("Error in the database when adding the component: {}", e),
        }
    }

    pub fn handle_add(&self, info: AddEntityCommand) {
        match info {
            AddEntityCommand::Machine(info) => self.handle_add_machine(info),
            AddEntityCommand::Component(info) => self.handle_add_component(info),
        }
    }
}
