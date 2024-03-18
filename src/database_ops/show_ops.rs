use super::DatabaseOperations;
use crate::args::{ShowEntity};
use rusqlite::{params, Error};
use upkeep::{Component, Log, Machine, PRIORITY_VEC, STATUS_VEC};

// TODO: implement the show operations
impl DatabaseOperations {
    pub fn handle_show(&self, show: ShowEntity) {
        
    }

}
