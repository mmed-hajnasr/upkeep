pub use rusqlite::{params, Connection, Result};
pub use chrono::NaiveDate;
pub mod tests;

pub const STATUS_VEC: [&str; 5] = ["optimal","improvable","unadjusted","suboptimal","defective"];
pub const PRIORITY_VEC: [&str; 4] = ["low","medium","high","critical"];

pub struct Machine {
    pub name: String,
    pub description: Option<String>,
}

pub struct Component {
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub priority: i32,
}

pub struct Log {
    pub status: i32,
    pub name: String,
    pub description: Option<String>,
    pub startdate: NaiveDate,
    pub fixed: bool,
    pub fixeddate: Option<NaiveDate>,
}