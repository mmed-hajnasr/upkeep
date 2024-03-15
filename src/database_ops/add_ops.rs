use super::database_operations;
use crate::args::{AddMachine, AddComponent};


impl database_operations {
    const add_machine_query: &str = "INSERT INTO machines (name, description) VALUES (?1, ?2)";
    fn handle_add_machine(info: AddMachine) {

    }
}