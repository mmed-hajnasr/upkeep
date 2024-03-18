use super::DatabaseOperations;
use crate::args::{EditComponent, EditEntityCommand, EditLog, EditMachine};
use rusqlite::params;

impl DatabaseOperations {
    fn handle_edit_machine(&self, info: EditMachine) {
        // check if the machine exists
        let machine = match self.get_machine(&info.name) {
            Some(machine) => machine,
            None => {
                println!("there is no machine with the name {}", info.name);
                return;
            }
        };

        // edit the machine
        const EDIT_MACHINE_QUERY: &str =
            "UPDATE Machine SET description = COALESCE(?1,description), name = COALESCE(?2,name) WHERE id = ?3";
        match self.conn.execute(
            EDIT_MACHINE_QUERY,
            params![info.new_description, info.new_name, machine.id],
        ) {
            Ok(_) => println!("Machine edited successfully."),
            Err(e) => println!("Error in the database when editing the machine: {}", e),
        }
    }

    fn handle_edit_component(&self, info: EditComponent) {
        // check if the component exists
        let component = match self.get_component(&info.name) {
            Some(component) => component,
            None => {
                println!("there is no component with the name {}", info.name);
                return;
            }
        };

        // edit the component
        const EDIT_COMPONENT_QUERY: &str = "UPDATE Component SET description = COALESCE(?1,description), name = COALESCE(?2,name), priority = COALESCE(?3,priority) WHERE id = ?4";
        match self.conn.execute(
            EDIT_COMPONENT_QUERY,
            params![
                info.new_description,
                info.new_name,
                info.new_priority,
                component.id
            ],
        ) {
            Ok(_) => println!("Component edited successfully."),
            Err(e) => println!("Error in the database when editing the component: {}", e),
        }
    }

    fn handle_edit_log(&self, info: EditLog) {
        // check if the log exists
        let log = match self.get_log(info.id) {
            Some(log) => log,
            None => {
                println!("there is no log with the id {}", info.id);
                return;
            }
        };

        // edit the log
        const EDIT_LOG_QUERY: &str = "UPDATE Log SET description = COALESCE(description,?1), status = COALESCE(status,?2), name = COALESCE(name,?3) WHERE id = ?4";
        match self.conn.execute(
            EDIT_LOG_QUERY,
            params![info.new_description, info.new_status, info.new_name, log.id],
        ) {
            Ok(_) => println!("Log edited successfully."),
            Err(e) => println!("Error in the database when editing the log: {}", e),
        }
    }

    pub fn handle_edit(&self, info: EditEntityCommand) {
        match info {
            EditEntityCommand::Component(info) => self.handle_edit_component(info),
            EditEntityCommand::Log(info) => self.handle_edit_log(info),
            EditEntityCommand::Machine(info) => self.handle_edit_machine(info),
        }
    }
}
