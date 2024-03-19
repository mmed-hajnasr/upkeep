#[cfg(test)]
mod test_database_opearations {
    use crate::args;
    use crate::database_ops::DatabaseOperations;
    use upkeep::{Component, Machine};
    #[test]
    fn test_add_edit_remove() {
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

        database_connection.handle_add(args::AddEntityCommand::Machine(machine1));
        database_connection.handle_add(args::AddEntityCommand::Machine(machine2));
        database_connection.handle_add(args::AddEntityCommand::Component(component1));
        database_connection.handle_add(args::AddEntityCommand::Component(component2));

        let machine1: Option<Machine> = database_connection.get_machine("machine1");
        let machine2: Option<Machine> = database_connection.get_machine("machine2");
        let component1: Option<Component> = database_connection.get_component("component1");
        let component2: Option<Component> = database_connection.get_component("component2");

        assert_eq!(
            machine1,
            Some(Machine {
                id: 1,
                name: "machine1".to_string(),
                description: Some("machine1 description".to_string()),
            })
        );

        assert_eq!(
            machine2,
            Some(Machine {
                id: 2,
                name: "machine2".to_string(),
                description: None,
            })
        );

        assert_eq!(
            component1,
            Some(Component {
                id: 1,
                name: "component1".to_string(),
                description: None,
                status: 1,
                priority: 1,
                error_date: None,
            })
        );

        assert_eq!(
            component2,
            Some(Component {
                id: 2,
                name: "component2".to_string(),
                description: Some("component2 description".to_string()),
                status: 1,
                priority: 3,
                error_date: None,
            })
        );

        let edit_machine1 = args::EditMachine {
            name: "machine1".to_string(),
            new_name: Some("machine_edited".to_string()),
            new_description: None,
        };
        let edit_machine2 = args::EditMachine {
            name: "machine2".to_string(),
            new_name: None,
            new_description: Some("machine2_edited".to_string()),
        };
        let edit_component1 = args::EditComponent {
            name: "component1".to_string(),
            new_name: Some("component_edited".to_string()),
            new_description: Some("new description".to_string()),
            new_priority: Some(3),
        };

        database_connection.handle_edit(args::EditEntityCommand::Machine(edit_machine1));
        database_connection.handle_edit(args::EditEntityCommand::Machine(edit_machine2));
        database_connection.handle_edit(args::EditEntityCommand::Component(edit_component1));

        let machine1: Option<Machine> = database_connection.get_machine("machine_edited");
        let machine2: Option<Machine> = database_connection.get_machine("machine2");
        let components: Option<Vec<Component>> =
            database_connection.get_components("machine_edited");

        assert_eq!(
            machine1,
            Some(Machine {
                id: 1,
                name: "machine_edited".to_string(),
                description: Some("machine1 description".to_string()),
            })
        );

        assert_eq!(
            machine2,
            Some(Machine {
                id: 2,
                name: "machine2".to_string(),
                description: Some("machine2_edited".to_string()),
            })
        );

        assert_eq!(
            components,
            Some(vec![Component {
                id: 1,
                name: "component_edited".to_string(),
                description: Some("new description".to_string()),
                status: 1,
                priority: 3,
                error_date: None,
            }])
        );

        let remove_component2 = args::RemoveComponent {
            name: "component2".to_string(),
        };
        let remove_machine = args::RemoveMachine {
            name: "machine_edited".to_string(),
        };

        database_connection.handle_remove(args::RemoveEntityCommand::Component(remove_component2));
        database_connection.handle_remove(args::RemoveEntityCommand::Machine(remove_machine));

        let component2 = database_connection.get_component("component2");
        let component1 = database_connection.get_component("component_edited");
        let machine = database_connection.get_machine("machine_edited");

        assert!(component2.is_none());
        assert!(component1.is_none());
        assert!(machine.is_none());
    }

    #[test]
    fn test_report_fix() {
        let database_connection = DatabaseOperations::new(":memory:");
        let machine1 = args::AddMachine {
            name: "machine1".to_string(),
            description: Some("machine1 description".to_string()),
        };
        let component1 = args::AddComponent {
            name: "component1".to_string(),
            machine: "machine1".to_string(),
            description: None,
            priority: Some(2),
        };
        let component2 = args::AddComponent {
            name: "component2".to_string(),
            machine: "machine1".to_string(),
            description: Some("component2 description".to_string()),
            priority: None,
        };

        let report_glich1 = args::ReportGlich {
            component: "component1".to_string(),
            name: Some("glich1".to_string()),
            description: Some("glich1 description".to_string()),
            status: None,
        };
        let report_glich2 = args::ReportGlich {
            component: "component1".to_string(),
            description: None,
            status: Some(4),
            name: None,
        };
        let report_glich3 = args::ReportGlich {
            component: "component2".to_string(),
            name: Some("glich3".to_string()),
            description: Some("glich2 description".to_string()),
            status: Some(3),
        };

        let fix_component2 = args::FixComponent {
            name: "component2".to_string(),
        };

        // add the entities
        database_connection.handle_add(args::AddEntityCommand::Machine(machine1));
        database_connection.handle_add(args::AddEntityCommand::Component(component1));
        database_connection.handle_add(args::AddEntityCommand::Component(component2));

        // report the gliches
        database_connection.handle_report_glich(report_glich1);
        database_connection.handle_report_glich(report_glich2);
        database_connection.handle_report_glich(report_glich3);

        // get the logs of component1
        let logs = database_connection.get_logs("component1");
        assert!(logs.is_some());
        let mut logs = logs.unwrap();

        // get the id of the log with the status 4
        logs.retain(|log| log.status == 4);
        assert_eq!(logs.len(), 1);

        let log_id = logs[0].id;

        let component1 = database_connection.get_component("component1").unwrap();
        let component2 = database_connection.get_component("component2").unwrap();

        assert_eq!(component1.status, 4);
        assert_eq!(component2.status, 3);

        database_connection.handle_fix(args::FixEntityCommand::Component(fix_component2));

        let component2 = database_connection.get_component("component2").unwrap();

        assert_eq!(component2.status, 1);

        database_connection.handle_fix(args::FixEntityCommand::Log(args::FixLog { id: log_id }));

        let component1 = database_connection.get_component("component1").unwrap();

        assert_eq!(component1.status, 2);

        let logs = database_connection.get_logs("component1").unwrap();
        assert_eq!(logs.len(), 2);

        for log in logs {
            database_connection.handle_remove(args::RemoveEntityCommand::Log(args::RemoveLog {
                id: log.id,
            }));
        }

        let logs = database_connection.get_logs("component1").unwrap();
        assert!(logs.len() == 0);

        let component1 = database_connection.get_component("component1").unwrap();

        assert_eq!(component1.status, 1);
    }
}
