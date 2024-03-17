#[cfg(test)]
mod test_database_opearations {
    use crate::database_ops::DatabaseOperations;
    use crate::args;
    #[test]
    fn test_add() {
        let database_connection = DatabaseOperations::new(":memory:");
        let machine1 = args::AddMachine {
            name: "machine1".to_string(),
            description: Some("machine1 description".to_string()),
        };
        let machine2 = args::AddMachine {
            name: "machine2".to_string(),
            description: None,
        };
        let component1 = args::AddComponent {
            name: "component1".to_string(),
            machine: "machine1".to_string(),
            description: None,
            priority: Some(1),
        };
        let component2 = args::AddComponent {
            name: "component2".to_string(),
            machine: "machine2".to_string(),
            description: Some("component2 description".to_string()),
            priority: None,
        };
        let component3 = args::AddComponent {
            name: "component3".to_string(),
            machine: "machine1".to_string(),
            description: Some("component3 description".to_string()),
            priority: Some(3),
        };
        database_connection.handle_add(component.command);
    }
}
