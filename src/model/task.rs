use crate::model::status::Status;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<u32>,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
            status: Status::Todo,
            started_at: None,
            completed_at: None,
        }
    }

    pub fn start(&mut self) {
        self.status = Status::Doing;
        self.started_at = Some(Utc::now());
    }

    pub fn complete(&mut self) {
        self.status = Status::Done;
        self.completed_at = Some(Utc::now());
    }
}
