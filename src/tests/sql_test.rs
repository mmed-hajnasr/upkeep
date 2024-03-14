#[cfg(test)]
mod tests {
    use crate::{params, Connection,Machine};

    #[test]
    fn test_machine() {
        let conn = Connection::open_in_memory().unwrap();

        // adding a machine
        let machine: Machine = Machine {
            name: "Machine 1".to_string(),
            description: Some("This is machine 1".to_string()),
        };

        // setting up the database
        match conn.execute_batch(include_str!("../sql/setup.sql")) {
            Ok(_) => {
                println!("Setup completed");
            }
            Err(e) => {
                println!("setup Error: {}", e);
            }
        };

        // insert the machine in the database
        match conn.execute(
            "Insert into Machine (name, description) values (?1, ?2)",
            params![machine.name, machine.description],
        ) {
            Ok(_) => {
                println!("Machine added");
            }
            Err(e) => {
                println!("Insert error: {}", e);
            }
        }

        // get the machine from the database
        let mut stmt = conn
            .prepare("SELECT name, description FROM Machine")
            .unwrap();

        // iterate over the machines
        let machine_iter = stmt.query_map(params![], |row| {
            Ok(Machine {
                name: row.get(0)?,
                description: row.get(1)?,
            })
        }).unwrap();

        machine_iter.for_each(|res_machine| {
            let machine = res_machine.unwrap();
            assert_eq!(machine.name, "Machine 1","Machine name is not correct");
            assert_eq!(machine.description.unwrap(), "This is machine 1","Machine description is not correct");
        });
    }
}
