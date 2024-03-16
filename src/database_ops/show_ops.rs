use super::DatabaseOperations;
use crate::args::{ShowAll, ShowComponent, ShowEntityCommand};
use crate::STATUS_VEC;
use rusqlite::{params, Error};

// TODO: implement the show operations
impl DatabaseOperations {
    fn handle_show_all(&self, info: ShowAll) {
        // get the list of machines
        const GET_ALL_MACHINES_QUERY: &str = "SELECT id,name FROM Machine";
        const GET_ONE_MACHINE_QUERY: &str = "SELECT id FROM Machine where name = ?1";
        let mut machines: Vec<(i32, String)> = Vec::new();
        match info.machine {
            Some(machine) => {
                match self
                    .conn
                    .query_row(GET_ONE_MACHINE_QUERY, params![machine], |row| {
                        row.get::<_, i32>(0)
                    }) {
                    Ok(id) => machines.push((id, machine)),
                    Err(Error::QueryReturnedNoRows) => {
                        panic!("Machine with the name {} does not exist.", machine)
                    }
                    Err(e) => panic!("Error in the database when getting the machine id: {}", e),
                }
            }
            None => {
                let mut stmt = self.conn.prepare(GET_ALL_MACHINES_QUERY).unwrap();
                let machine_iter = stmt
                    .query_map(params![], |row| {
                        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
                    })
                    .unwrap();
                for machine in machine_iter {
                    machines.push(machine.unwrap());
                }
            }
        }

        // get the list of components
        const GET_ALL_COMPONENTS_QUERY: &str =
            "SELECT name,description,status FROM Component WHERE machineid = ?1 ORDER BY priority";
        for machine in machines {
            println!("{}:", machine.1);
            let mut stmt = self.conn.prepare(GET_ALL_COMPONENTS_QUERY).unwrap();
            let component_iter = stmt
                .query_map(params![machine.0], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, usize>(2)?,
                    ))
                })
                .unwrap();
            for component in component_iter {
                let component = component.unwrap();
                println!(
                    "  {}({}){}",
                    component.0,
                    STATUS_VEC[component.2 - 1],
                    component
                        .1
                        .as_ref()
                        .map(|desc| ": ".to_owned() + desc)
                        .unwrap_or("".to_string())
                );
            }
        }
    }

    fn handle_show_component(&self, info: ShowComponent) {
        // check if the component name exists
        const CHECK_COMPONENT_NAME_QUERY: &str = "SELECT id FROM Component WHERE name = ?1";
        let id: i32 =
            match self
                .conn
                .query_row(CHECK_COMPONENT_NAME_QUERY, params![info.name], |row| {
                    row.get::<_, i32>(0)
                }) {
                Ok(id) => id,
                Err(Error::QueryReturnedNoRows) => {
                    panic!("Component with the name {} does not exist.", info.name)
                }
                Err(e) => panic!(
                    "Error in the database when checking for the component name: {}",
                    e
                ),
            };

        // get the componen logs
        const GET_COMPONENT_LOGS_QUERY: &str =
            "SELECT name,description,status FROM Log WHERE componentid = ?1 and fixed = 0 ORDER BY status DESC, startDate DESC;";
        let mut stmt = self.conn.prepare(GET_COMPONENT_LOGS_QUERY).unwrap();
        let log_iter: Vec<_> = stmt
            .query_map(params![id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, usize>(2)?,
                ))
            })
            .unwrap()
            .collect();
        if &log_iter.len() == &0 {
            println!("No logs for the component {}.", info.name);
        } else {
            println!("Logs for the component {}:", info.name);
            log_iter.iter().for_each(|log| {
                let log = log.as_ref().unwrap();
                println!(
                    "  {}({}){}",
                    log.0,
                    STATUS_VEC[log.2 - 1],
                    log.1
                        .as_ref()
                        .map(|desc| ": ".to_owned() + desc)
                        .unwrap_or("".to_string())
                );
            });
        }
    }

    pub fn handle_show(&self, info: ShowEntityCommand) {
        match info {
            ShowEntityCommand::All(info) => self.handle_show_all(info),
            ShowEntityCommand::Component(info) => self.handle_show_component(info),
        }
    }
}
