use super::DatabaseOperations;
use crate::args::ShowEntity;
use colored::ColoredString;
use upkeep::status_vec;

// TODO: implement the show operations
impl DatabaseOperations {
    pub fn handle_show(&self, info: ShowEntity) {
        if let Some(component_name) = info.component.clone() {
            self.show_component(info.clone(), "".to_string(), &component_name);
            return;
        }
        if let Some(machine_name) = info.machine.clone() {
            self.show_machine(info.clone(), "".to_string(), &machine_name);
            return;
        }
        let machines = self.get_machines();
        if machines.is_empty() {
            println!("No machines found.");
            return;
        }
        for machine in machines {
            self.show_machine(info.clone(), "".to_string(), &machine.name);
        }
    }
    fn show_machine(&self, info: ShowEntity, mut prefix: String, machine_name: &str) {
        // get the machine
        let machine = match self.get_machine(machine_name) {
            Some(machine) => machine,
            None => {
                println!("{} machine {} was not found", prefix, machine_name);
                return;
            }
        };

        // print the machine
        let description = if !info.desc {
            "".to_string()
        } else {
            match machine.description {
                Some(description) => format!(": {}", description),
                None => "".to_string(),
            }
        };
        println!("{}-{}{}", prefix, machine.name, description);

        // get the components and print them
        let components = self.get_components(&machine.name);
        prefix.push_str("       ");

        let mut components = match components {
            Some(components) => components,
            None => {
                println!("{}This machine has no components.", prefix);
                return;
            }
        };

        components.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| b.status.cmp(&a.status))
                .then_with(|| b.error_date.cmp(&a.error_date))
        });

        for component in components {
            self.show_component(info.clone(), prefix.clone(), &component.name);
        }
    }

    fn show_component(&self, info: ShowEntity, mut prefix: String, component_name: &str) {
        let status_vec: [ColoredString; 6] = status_vec();

        // get the component
        let component = match self.get_component(component_name) {
            Some(component) => component,
            None => {
                println!("{} component {} does not exist.", prefix, component_name);
                return;
            }
        };

        // print the component
        let description = if !info.desc {
            "".to_string()
        } else {
            match component.description {
                Some(description) => format!(": {}", description),
                None => "".to_string(),
            }
        };
        println!(
            "{}-{}{}{}",
            prefix, component.name, status_vec[component.status as usize], description
        );

        if !(info.logs || info.all) {
            return;
        }

        // process the logs
        prefix.push_str("       ");
        let mut logs = self.get_logs(&component.name).unwrap();

        // filter the logs
        if info.logs {
            logs = logs.into_iter().filter(|log| !log.fixed).collect();
        }

        if logs.is_empty() {
            println!("{}This component has no logs.", prefix);
            return;
        }

        // sort the logs
        if info.sort_oldest {
            logs.sort_by(|a, b| {
                a.fixed
                    .cmp(&b.fixed)
                    .then_with(|| a.start_date.cmp(&b.start_date))
            });
        } else if info.sort_newest {
            logs.sort_by(|a, b| {
                a.fixed
                    .cmp(&b.fixed)
                    .then_with(|| b.start_date.cmp(&a.start_date))
            });
        } else {
            logs.sort_by(|a, b| {
                a.fixed
                    .cmp(&b.fixed)
                    .then_with(|| b.status.cmp(&a.status))
                    .then_with(|| a.start_date.cmp(&b.start_date))
            });
        }

        // print the logs
        for log in logs {
            let description = if info.desc {
                match log.description {
                    Some(description) => format!(": {}", description),
                    None => "".to_string(),
                }
            } else {
                "".to_string()
            };

            println!(
                "{}-({}){}{}{}",
                prefix,
                log.id,
                log.name,
                if !log.fixed {
                    &status_vec[log.status as usize]
                } else {
                    &status_vec[0]
                },
                description
            );
        }
    }
}
