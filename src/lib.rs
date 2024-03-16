use chrono::NaiveDate;


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