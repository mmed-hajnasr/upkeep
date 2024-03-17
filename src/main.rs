mod args;
mod database_ops;
mod tests;
use std::path::Path;

use clap::Parser;

fn main() {
    let dir = Path::new("target");
    let path = dir.join("test.db");
    let database_connection = database_ops::DatabaseOperations::new(path.to_str().unwrap());

    let command = args::UpkeepArgs::parse();
    match command.action {
        args::Action::Add(info) => database_connection.handle_add(info.command),
        // args::Action::Fix(info) => todo!(),
        // args::Action::Remove(info) => todo!(),
        // args::Action::Edit(info) => todo!(),
        args::Action::Show(info) => database_connection.handle_show(info.command),
        args::Action::Report(info) => database_connection.handle_report_glich(info),
        // args::Action::Dashboard => todo!(),
    }
}
