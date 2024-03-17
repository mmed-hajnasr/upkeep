use chrono::NaiveDateTime;

pub const STATUS_VEC: [&str; 5] = [
    "optimal",
    "improvable",
    "unadjusted",
    "suboptimal",
    "defective",
];
pub const PRIORITY_VEC: [&str; 4] = ["low", "medium", "high", "critical"];

#[derive(Debug)]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}

#[derive(Debug)]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub priority: i32,
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.description == other.description
            && self.status == other.status
            && self.priority == other.priority
    }
}

#[derive(Debug)]
pub struct Log {
    pub id: i32,
    pub status: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub fixed: bool,
    pub fixed_date: Option<NaiveDateTime>,
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.name == other.name
            && self.description == other.description
            && self.start_date == other.start_date
            && self.fixed == other.fixed
            && self.fixed_date == other.fixed_date
    }
}
