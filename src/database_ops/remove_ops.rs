use super::DatabaseOperations;
use crate::args::{RemoveComponent, RemoveEntityCommand, RemoveLog, RemoveMachine};
use rusqlite::params;

impl DatabaseOperations {
    pub fn handle_remove(&self, command: RemoveEntityCommand) {
        match command {
            RemoveEntityCommand::Machine(machine) => self.handle_remove_machine(machine),
            RemoveEntityCommand::Component(component) => self.handle_remove_component(component),
            RemoveEntityCommand::Log(log) => self.handle_remove_log(log),
        }
    }

    fn handle_remove_machine(&self, machine: RemoveMachine) {
        // check if the machine exists
        let machine = match self.get_machine(&machine.name) {
            Some(machine) => machine,
            None => {
                println!("Machine {} does not exist", machine.name);
                return;
            }
        };

        // remove the machine
        self.conn
            .execute("DELETE FROM Machine WHERE id = ?1", params![machine.id])
            .expect(&format!(
                "Error removing the machine {} from the database",
                machine.name
            ));
        println!("Machine {} removed", machine.name);
    }

    fn handle_remove_component(&self, component: RemoveComponent) {
        // check if the component exists
        let component = match self.get_component(&component.name) {
            Some(component) => component,
            None => {
                println!("Component {} does not exist", component.name);
                return;
            }
        };

        // remove the component
        self.conn
            .execute("DELETE FROM Component WHERE id = ?1", params![component.id])
            .expect(&format!(
                "Error removing the component {} from the database",
                component.name
            ));
        println!("Component {} removed", component.name);
    }

    fn handle_remove_log(&self, log: RemoveLog) {
        // check if the log exists
        let log = match self.get_log(log.id) {
            Some(log) => log,
            None => {
                println!("Log with id {} does not exist", log.id);
                return;
            }
        };

        // remove the log
        self.conn
            .execute("DELETE FROM Log WHERE id = ?1", params![log.id])
            .expect(&format!(
                "Error removing the log {} from the database",
                log.name
            ));
        println!("Log {} removed", log.name)
    }
}
