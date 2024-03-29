use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct UpkeepArgs {
    #[clap(subcommand)]
    pub action: Action,
}
//* the action commands
#[derive(Debug, Subcommand)]
pub enum Action {
    /// Mark a component or a log as fixed.
    Fix(FixEntity),

    /// add a machine or a component.
    Add(AddEntity),

    /// remove a machine or a component or a log.
    Remove(RemoveEntity),

    /// edit the attributes of a machine or a component or a log.
    Edit(EditEntity),

    /// show the current state of your machines and components.
    Show(ShowEntity),

    /// report a glich.
    Report(ReportGlich),
    // open the dashboard.
    // Dashboard,
}

//* the entity commands
//* the fix commands
#[derive(Debug, Args)]
pub struct FixEntity {
    #[clap(subcommand)]
    pub command: FixEntityCommand,
}

#[derive(Debug, Subcommand)]
pub enum FixEntityCommand {
    /// fix all the logs of a component.
    Component(FixComponent),

    /// fix a log.
    Log(FixLog),
}

#[derive(Debug, Args)]
pub struct FixComponent {
    /// The name of the component that will be fixed.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct FixLog {
    /// the id of the log that will be fixed.
    pub id: i32,
}

//* the show commands
#[derive(Debug, Args, Clone)]
pub struct ShowEntity {

    /// the name of the machine to be shown.
    #[clap(short, long, group = "element")]
    pub machine: Option<String>,

    /// the name of the component to be shown.
    #[clap(short, long, group = "element")]
    pub component: Option<String>,

    /// show the unfixed logs.
    #[clap(short, long)]
    pub logs: bool,

    /// show all the logs.
    #[clap(short, long)]
    pub all: bool,

    /// show the description.
    #[clap(short, long)]
    pub desc: bool,

    /// sort the logs by the oldest.
    #[clap(short = 'o', long, group = "sort")]
    pub sort_oldest: bool,

    /// sort the logs by the newest.
    #[clap(short = 'n', long, group = "sort")]
    pub sort_newest: bool,

}
//* the add commands
#[derive(Debug, Args)]
pub struct AddEntity {
    #[clap(subcommand)]
    pub command: AddEntityCommand,
}

#[derive(Debug, Subcommand)]
pub enum AddEntityCommand {
    /// add a machine.
    Machine(AddMachine),

    /// add a component.
    Component(AddComponent),
}

#[derive(Debug, Args)]
pub struct AddMachine {
    /// the name of the machine that will be added.
    pub name: String,

    /// the description of the machine that will be added.
    #[clap(short, long)]
    pub description: Option<String>,
}

#[derive(Debug, Args)]
pub struct AddComponent {
    /// the name of the component that will be added.
    pub name: String,

    /// the name of the machine to which the component will be added.
    pub machine: String,

    /// the description of the component that will be added.
    #[clap(short, long)]
    pub description: Option<String>,

    /// the priority of the component that will be added.
    #[clap(short, long)]
    pub priority: Option<i32>,
}

//* the remove commands
#[derive(Debug, Args)]
pub struct RemoveEntity {
    #[clap(subcommand)]
    pub command: RemoveEntityCommand,
}

#[derive(Debug, Subcommand)]
pub enum RemoveEntityCommand {
    /// remove a machine.
    Machine(RemoveMachine),

    /// remove a component.
    Component(RemoveComponent),

    /// remove a log.
    Log(RemoveLog),
}

#[derive(Debug, Args)]
pub struct RemoveMachine {
    /// the name of the machine that will be removed.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RemoveComponent {
    /// the name of the component that will be removed.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RemoveLog {
    /// the id of the log that will be removed.
    pub id: i32,
}

//* the edit commands
#[derive(Debug, Args)]
pub struct EditEntity {
    #[clap(subcommand)]
    pub command: EditEntityCommand,
}

#[derive(Debug, Subcommand)]
pub enum EditEntityCommand {
    /// edit a machine.
    Machine(EditMachine),

    /// edit a component.
    Component(EditComponent),

    /// edit a log.
    Log(EditLog),
}

#[derive(Debug, Args)]
pub struct EditMachine {
    /// the name of the machine that will be edited.
    pub name: String,

    /// the new name of the machine.
    #[clap(short, long)]
    pub new_name: Option<String>,

    /// the new description of the machine.
    #[clap(short='d', long)]
    pub new_description: Option<String>,
}

#[derive(Debug, Args)]
pub struct EditComponent {
    /// the name of the component that will be edited.
    pub name: String,

    /// the new name of the component.
    #[clap(short = 'n', long)]
    pub new_name: Option<String>,

    /// the new description of the component.
    #[clap(short = 'd', long)]
    pub new_description: Option<String>,

    /// the new priority of the component.
    #[clap(short = 'p', long)]
    pub new_priority: Option<i32>,
}

#[derive(Debug, Args)]
pub struct EditLog {
    /// the id of the log that will be edited.
    pub id: i32,

    /// the new description of the log.
    #[clap(short = 'd', long)]
    pub new_description: Option<String>,

    /// the new status of the log.
    #[clap(short = 's', long)]
    pub new_status: Option<i32>,

    /// the new name of the log.
    #[clap(short, long)]
    pub new_name: Option<String>,
}

//* the report command
#[derive(Debug, Args)]
pub struct ReportGlich {
    /// the name of the component that has the glich.
    pub component: String,

    /// the description of the glich.
    #[clap(short, long)]
    pub description: Option<String>,

    // TODO: turn this into a value enum
    /// the status of the component: 1 for optimal, 2 for improvable, 3 for unadjusted, 4 for suboptimal, 5 for defective.
    #[clap(short, long)]
    pub status: Option<i32>,

    /// the name of the log that describes the glich.
    #[clap(short, long)]
    pub name: Option<String>,
}
