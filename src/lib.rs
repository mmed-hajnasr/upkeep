use chrono::NaiveDateTime;


pub struct Machine {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

pub struct Component {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub priority: i32,
}

pub struct Log {
    pub id: i32,
    pub status: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub fixed: bool,
    pub fixed_date: Option<NaiveDateTime>,
}
