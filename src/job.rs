use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub execution_time: i64,
    pub priority: u8,
    pub description: String,
    pub function: String,
    pub status: Status,
}

impl Job {
    pub fn new(
        execution_time: i64,
        priority: u8,
        description: impl Into<String>,
        function: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            execution_time,
            priority,
            description: description.into(),
            function: function.into(),
            status: Status::Pending,
        }
    }
}
