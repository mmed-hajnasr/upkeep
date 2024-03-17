#[cfg(test)]
mod test_database_opearations {
    use crate::args;
    use crate::database_ops::DatabaseOperations;
    use upkeep::{Component, Machine};
    #[test]
    fn test_add() {
        let database_connection = DatabaseOperations::new(":memory:");

        assert!(database_connection.get_machine("machine1").is_none());

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

        database_connection.handle_add_machine(machine1);
        database_connection.handle_add_machine(machine2);
        database_connection.handle_add_component(component1);
        database_connection.handle_add_component(component2);

        let machine1: Option<Machine> = database_connection.get_machine("machine1");
        let machine2: Option<Machine> = database_connection.get_machine("machine2");
        let component1: Option<Component> = database_connection.get_component("component1");
        let component2: Option<Component> = database_connection.get_component("component2");

        assert_eq!(machine1 , Some(Machine {
            id: 1,
            name: "machine1".to_string(),
            description: Some("machine1 description".to_string()),
        }));

        assert_eq!(machine2 , Some(Machine {
            id: 2,
            name: "machine2".to_string(),
            description: None,
        }));

        assert_eq!(component1 , Some(Component {
            id: 1,
            name: "component1".to_string(),
            description: None,
            status: 1,
            priority: 1,
        }));

        assert_eq!(component2 , Some(Component {
            id: 2,
            name: "component2".to_string(),
            description: Some("component2 description".to_string()),
            status: 1,
            priority: 3,
        }));

    }
}
