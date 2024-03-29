use dirs;

use clap::Parser;
use upkeep::args;
use upkeep::database_ops;

fn main() {
    let data_dir = dirs::data_dir().unwrap().join("upkeep");
    let database_connection = database_ops::DatabaseOperations::new(data_dir);

    let command = args::UpkeepArgs::parse();
    match command.action {
        args::Action::Add(info) => database_connection.handle_add(info.command),
        args::Action::Fix(info) => database_connection.handle_fix(info.command),
        args::Action::Remove(info) => database_connection.handle_remove(info.command),
        args::Action::Edit(info) => database_connection.handle_edit(info.command),
        args::Action::Show(info) => database_connection.handle_show(info),
        args::Action::Report(info) => database_connection.handle_report_glich(info),
        // args::Action::Dashboard => todo!(),
    }
}
