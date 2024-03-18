use super::DatabaseOperations;
use crate::args::{FixComponent, FixEntityCommand, FixLog};
use rusqlite::params;

impl DatabaseOperations {
    fn handle_fix_component(&self, info: FixComponent) {
        let mut logs = match self.get_logs(info.name.as_str()) {
            Some(logs) => logs,
            None => {
                println!("there is no component with the name {}", info.name);
                return;
            }
        };

        if logs.len() == 0 {
            println!("there are no logs for the component {}", info.name);
            return;
        }

        logs.retain(|log| log.fixed == false);

        if logs.len() == 0 {
            println!("there are no logs to fix for the component {}", info.name);
            return;
        }

        let mut count: i32 = 0;
        for log in logs {
            self.handle_fix_log(FixLog { id: log.id });
            count += 1;
        }
        println!("{} gliches were fixed successfully.", count);
    }

    fn handle_fix_log(&self, info: FixLog) {
        let log = match self.get_log(info.id) {
            Some(log) => log,
            None => {
                println!("there is no log with the id {}", info.id);
                return;
            }
        };

        if log.fixed {
            println!("the log with the id {} is already fixed.", info.id);
            return;
        }

        const FIX_LOG_QUERY: &str = "UPDATE Log SET fixed = 1 WHERE id = ?1";
        match self.conn.execute(FIX_LOG_QUERY, params![info.id]) {
            Ok(_) => println!("the log {} was fixed successfully.", log.name),
            Err(e) => panic!("Error in the database when fixing the log: {}", e),
        }
    }

    pub fn handle_fix(&self, info: FixEntityCommand) {
        match info {
            FixEntityCommand::Component(info) => self.handle_fix_component(info),
            FixEntityCommand::Log(info) => self.handle_fix_log(info),
        }
    }
}
