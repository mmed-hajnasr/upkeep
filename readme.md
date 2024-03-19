# upkeep

## Project Overview

Description: A command-line tool for logging and tracking machine and component statuses for later maintenance.

## Functionality

- upkeep allows users to create a record of issues or problems encountered with different components within a machine.
- Users can then manage these logs, marking them as fixed or removing them once addressed.
- This helps maintain a record of potential maintenance needs for later attention.

## installation

### using cargo

```bash
cargo install upkeep
```

### from source

```bash
git clone https://github.com/mmed-hajnasr/upkeep.git
cd upkeep
cargo install --path .
```

## Usage

upkeep is a command-line tool.
It provides various subcommands for managing your logs:

- fix: Marks a component or log as fixed.
- add: Adds a new machine or component to the log.
- remove: Removes a machine, component, or log entry.
- edit: Edits the details of a machine, component, or log entry.
- show: Displays the current status of your machines and components.
- report: Reports a new glitch.
- help: Provides help information for the main command or specific subcommands.

## Dependencies

- clap: Used for parsing command-line arguments.
- colored: Used for adding color formatting to the output for better readability.
- rusqlite: Used for interacting with a SQLite database to store the log information persistently.
